use crate::{
    prelude::*,
    base::index::ZoneInd,
    comm::{
        msg::OzoneMsg,
        response::Responder,
    },
    data::{
        core::{
            Encode,
        },
    },
};

use oxedize_fe2o3_iop_db::api::{
    RestSchemesOverride,
};
use oxedize_fe2o3_jdat::{
    prelude::*,
    id::NumIdDat,
};
//use oxedize_fe2o3_hash::{
//    csum::{
//        ChecksummerDefAlt,
//        ChecksumScheme,
//    },
//};

use std::time::Instant;

pub fn compare_values(i: usize, v1: &Vec<u8>, vorig: &Dat) -> Outcome<()> {
    // Get the associated data since we know vorig comes with a Dat byte wrapper
    if let Some(v2) = vorig.bytes_ref() {
        if v1.len() != v2.len() {
            return Err(err!(errmsg!(
                "Data {}: Original length = {}, retrieved length = {}.",
                i, v2.len(), v1.len(),
            ))); 
        }
        for j in 0..v1.len() {
            if v1[j] != v2[j] {
                debug!("i        = {}",i);
                debug!("expected = {:02x?}",v2);
                debug!("got      = {:02x?}",v1);
                return Err(err!(errmsg!(
                    "Data {}: Differs at position {}.", i, j,
                ))); 
            }
        }
    } else {
        return Err(err!(errmsg!(
            "Given test value with index {} is not a Dat containing bytes.", i,
        ), Invalid, Input)); 
    }
    Ok(())
}

/// Identifies sequences that are unique, starting from the last element.
pub fn find_unique(v: &Vec<Vec<u8>>) -> Vec<bool> {
    let mut b = vec![true; v.len()];
    if v.len() > 1 {
        for i in (1..v.len()).rev() {
            for j in 0..i {
                if v[i].len() == v[j].len() {
                    let mut equal = true;
                    for k in 0..v[i].len() {
                        if v[i][k] != v[j][k] {
                            equal = false;
                            break;
                        }
                    }
                    if equal {
                        b[j] = false;
                    }
                }
            }
        }
    }
    b
}

pub fn stopwatch(
    elapsed:    f64,
    n:          usize,
    byts:       usize, // total bytes in data
)
    -> (u64, u64)
{
    let tps = (n as f64) / elapsed;
    let bw = ((byts as f64) / elapsed) * (8.0 / 1_000_000.0);
    test!("Performance metrics:");
    test!("  Elapsed:     {:10.4} [s]", elapsed);
    test!("  TPS:         {:10.2}", tps);
    test!("  Bandwidth:   {:10.3} [Mb]/[s]", bw);

    (tps as u64, bw as u64)
}

pub fn encode_daticles(
    ks:         Vec<Dat>,
    vs:         Vec<Dat>,
    byts:       usize, // total bytes in data
)
    -> Outcome<(Vec<Vec<u8>>, Vec<Vec<u8>>, u64, u64)>
{
    let n = ks.len();

    test!("Start daticle encoding timing run...");
    let start = Instant::now();
    for i in 0..n {
        let (_k, _v) = res!(Encode::encode_dat(ks[i].clone(), vs[i].clone()));
    }
    let elapsed = start.elapsed().as_secs_f64();
    let (tps, bw) = stopwatch(elapsed, n, byts);

    test!("Redoing encoding to collect bytes...");
    let mut kbyts = Vec::new();
    let mut vbyts = Vec::new();
    for i in 0..n {
        let (k, v) = res!(Encode::encode_dat(ks[i].clone(), vs[i].clone()));
        kbyts.push(k);
        vbyts.push(v);
    }
    test!("  Finished.");

    Ok((kbyts, vbyts, tps, bw))
}

//pub fn encode_write_messages<
//    const UIDL: usize,
//    UID: NumIdDat<UIDL> + 'static,
//    C: Checksummer + 'static,
//>(
//    meta:       &Meta<UIDL, UID>,
//    ks:         Vec<Vec<u8>>,
//    vs:         Vec<Vec<u8>>,
//    byts:       usize, // total bytes in data
//    csummer:    ChecksummerDefAlt<ChecksumScheme, C>,
//)
//    -> Outcome<(u64, u64)>
//{
//    let n = ks.len();
//
//    test!("Assemble wrapped KeyVals...");
//    let mut keyvals = Vec::new();
//    for i in 0..n {
//        let mut chash = [0u8; 4];
//        for j in 0..4 {
//            chash[j] = ks[i][j];
//        }
//        let kv = KeyVal {
//            key:    Key::Complete(ks[i].clone()),
//            val:    vs[i].clone(),
//            chash, // dummy
//            meta:   meta.clone(),
//            cbpind: 3, // dummy
//        };
//        keyvals.push(kv);
//    }
//    test!("Start write message encoding timing run...");
//    let start = Instant::now();
//    for kv in keyvals {
//        res!(Encode::encode(kv, csummer.clone()));
//    }
//    let elapsed = start.elapsed().as_secs_f64();
//    let (tps, bw) = stopwatch(elapsed, n, byts);
//
//    Ok((tps, bw))
//}

pub fn prepare_write_messages<
    const UIDL: usize,
    UID:    NumIdDat<UIDL> + 'static,
    ENC:    Encrypter + 'static,
    KH:     Hasher + 'static,
	PR:     Hasher + 'static,
    CS:     Checksummer + 'static,
>(
    db:         &mut O3db<UIDL, UID, ENC, KH, PR, CS>,
    user:       UID,
    schms2:     Option<&RestSchemesOverride<ENC, KH>>,
    ks:         Vec<Dat>,
    vs:         Vec<Dat>,
    byts:       usize, // total bytes in data
)
    -> Outcome<(usize, Vec<(OzoneMsg<UIDL, UID, ENC, KH>, ZoneInd)>)>
{
    let n = ks.len();

    test!("Encoding data for writing...");
    let mut msgs = Vec::new();
    let resp = Responder::none(None);
    let start = Instant::now();
    for i in 0..n {
        let msgs2 = res!(db.api().prepare_write_dat(
            ks[i].clone(),
            vs[i].clone(),
            user,
            schms2,
            resp.clone(),
        ));
        for msg in msgs2 {
            msgs.push(msg);
        }
    }
    let elapsed = start.elapsed().as_secs_f64();
    test!("  n = {}, msgs.len = {}", n, msgs.len());
    let tps = (n as f64) / elapsed;
    let bw = ((byts as f64) / elapsed) * (8.0 / 1_000_000.0);
    test!("Write preparation performance metrics:");
    test!("  Elapsed:     {:10.4} [s]", elapsed);
    test!("  TPS:         {:10.2}", tps);
    test!("  Bandwidth:   {:10.3} [Mb]/[s]", bw);

    Ok((n, msgs))
}
