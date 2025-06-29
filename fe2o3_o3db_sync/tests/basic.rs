use oxedyne_fe2o3_core::{
    prelude::*,
    alt::Override,
    rand::Rand,
};
use oxedyne_fe2o3_crypto::enc::EncryptionScheme;
use oxedyne_fe2o3_hash::{
    csum::ChecksumScheme,
    hash::HashScheme,
};
use oxedyne_fe2o3_iop_db::api::{
    Meta,
    RestSchemesOverride,
};
use oxedyne_fe2o3_jdat::{
    prelude::*,
};
use oxedyne_fe2o3_o3db_sync::{
    base::{
        constant,
        index::ZoneInd,
    },
    comm::{
        response::Wait,
    },
    data::core::RestSchemesInput,
    file::zdir::ZoneDir,
    test::{
        dbapi,
        file::{
            delete_all_index_files,
            corrupt_an_index_file,
        },
        setup,
    },
};
use oxedyne_fe2o3_test::error::delayed_error;

use std::{
    collections::BTreeMap,
    path::Path,
    thread,
    time::Duration,
};


const wait: Wait = constant::USER_REQUEST_WAIT;

pub fn test_basic(_filter: &'static str) -> Outcome<()> {

    let db_root = res!(Path::new("./test_db").canonicalize());
    //              +         +         +         +
    //              1234567890123456789012345678901234
    let key = dat!("A long key exceeding 32 bytes");
    let mut valvec = vec![0u8; 5_000];
    Rand::fill_u8(&mut valvec[..]);
    let val = dat!(valvec.clone());

    let mut enckey = [0u8; 32];
    Rand::fill_u8(&mut enckey);
    let aes_gcm = res!(EncryptionScheme::new_aes_256_gcm_with_key(&enckey[..]));
    let _sha3_256 = HashScheme::new_sha3_256();
    let crc32 = ChecksumScheme::new_crc32();
    let schms2: RestSchemesOverride<EncryptionScheme, HashScheme> =
        RestSchemesOverride::default().set_encrypter(Override::Default(aes_gcm.clone()));
    let schms2 = Some(&schms2);
    let user = setup::Uid::default();
    //let meta = Meta::<{ setup::UID_LEN }, setup::Uid>::new(setup::Uid::default());
    let schms_input = RestSchemesInput::new(
        Some(aes_gcm.clone()),
        None::<HashScheme>,
        None::<HashScheme>,
        Some(crc32.clone()),
    );
    //let user1_name = fmt!("Alice83");
    //let user1_email = fmt!("alice83@gmail.com");
    //let user1_pass = fmt!("alice_pass");

    let mut cfg = res!(setup::default_cfg());
    cfg.cache_size_limit_bytes  = 100_000;
    cfg.rest_chunk_threshold    = 700;
    cfg.num_cbots_per_zone      = 2;
    cfg.num_zones               = 3;
    cfg.zone_overrides          = mapdat!{
        1u16    =>  mapdat!{
            "dir"       =>  "../test_db_zone_container",
            "max_size"  =>  100u64,
        },
        3u16    =>  mapdat!{
            "dir"       =>  "",
            "max_size"  =>  100u64,
        },
    }.get_map().unwrap();

    let error_delay = 2;

    {
        test!(sync_log::stream(), "+---------------------------------------------+");
        test!(sync_log::stream(), "| NEW OZONE SESSION                           |");
        test!(sync_log::stream(), "| Wipe all traces of previous test.           |");
        test!(sync_log::stream(), "| Start database.                             |");
        test!(sync_log::stream(), "| Store and fetch some simple data.           |");
        test!(sync_log::stream(), "| Store and fetch some chunked data:          |");
        test!(sync_log::stream(), "|  * Including one cycle wiping the cache.    |");
        test!(sync_log::stream(), "| Gracefully shut down the database.          |");
        test!(sync_log::stream(), "+---------------------------------------------+");
        // Wipe all traces of previous test.
        // Start database.
        let mut db = match setup::start_db(
            db_root.clone(),
            Some(cfg.clone()),
            schms_input.clone(),
            Some(fmt!("./test_db_zone_container")),
            true,
            true,
        ) {
            // These pauses on errors are needed to capture tardy messages from asynchronous
            // logging.
            Err(e) => return Err(delayed_error(e, error_delay)),
            Ok(db) => db,
        };

        thread::sleep(Duration::from_secs(1));

        // Store and fetch some simple data.
        match dbapi::simple(&mut db, schms2, user) {
            Err(e) => return Err(delayed_error(e, error_delay)),
            _ => (),
        }

        // Store and fetch some simple data.
        match dbapi::simple_api(&mut db, user) {
            Err(e) => return Err(delayed_error(e, error_delay)),
            _ => (),
        }

        // Store and fetch some chunked data:
        // * Including one cycle wiping the cache.
        match dbapi::store_chunked_data(
            &mut db,
            schms2,
            user,
            key.clone(),
            val.clone(),
        ) {
            Err(e) => return Err(delayed_error(e, error_delay)),
            _ => (),
        };

        //res!(db.dump_caches(constant::USER_REQUEST_WAIT));

        // While debugging, extra work can mean we start reading data before bots have written it.
        thread::sleep(Duration::from_secs(2));
        match dbapi::fetch_chunked_data(
            &mut db,
            &key,
            &valvec,
            user,
            schms2,
        ) {
            Err(e) => return Err(delayed_error(e, error_delay)),
            _ => (),
        }
        test!(sync_log::stream(), "Listing files...");
        res!(db.api().list_files(wait));
        //res!(db.dump_caches(constant::USER_REQUEST_WAIT));
        test!(sync_log::stream(), "Shutting db down...");
        // Gracefully shut down the database.
        res!(db.shutdown());
    }

    thread::sleep(Duration::from_secs(1));

    let zdirs: BTreeMap<ZoneInd, ZoneDir>;

    {
        test!(sync_log::stream(), "+---------------------------------------------+");
        test!(sync_log::stream(), "| NEW OZONE SESSION                           |");
        test!(sync_log::stream(), "| Start database:                             |");
        test!(sync_log::stream(), "|  * Including caching index files.           |");
        test!(sync_log::stream(), "| Fetch chunked data from previous session.   |");
        test!(sync_log::stream(), "| Gracefully shut down the database.          |");
        test!(sync_log::stream(), "+---------------------------------------------+");
        let mut db = match setup::start_db(
            db_root.clone(),
            Some(cfg.clone()),
            schms_input.clone(),
            Some(fmt!("./test_db_zone_container")),
            true,
            false,
        ) {
            Err(e) => return Err(delayed_error(e, error_delay)),
            Ok(db) => db,
        };

        thread::sleep(Duration::from_secs(1));

        zdirs = res!(db.api().get_zone_dirs());
        //res!(db.dump_caches(constant::USER_REQUEST_WAIT));
        match dbapi::fetch_chunked_data(
            &mut db,
            &key,
            &valvec,
            user,
            schms2,
        ) {
            Err(e) => return Err(delayed_error(e, error_delay)),
            _ => (),
        }

        test!(sync_log::stream(), "Demonstrating collecting the state of ozone resources, ");
        test!(sync_log::stream(), "which is regularly reported by each zone to the supervisor.");
        let zstats = res!(db.api().ozone_state(constant::USER_REQUEST_WAIT));
        for (i, zstat) in zstats.iter().enumerate() {
            test!(sync_log::stream(), "Zone {} {:?}", i+1, zstat);
        }

        test!(sync_log::stream(), "Shutting db down...");
        res!(db.shutdown());
    }

    thread::sleep(Duration::from_secs(1));

    res!(delete_all_index_files(&zdirs));

    {
        test!(sync_log::stream(), "+---------------------------------------------+");
        test!(sync_log::stream(), "| NEW OZONE SESSION                           |");
        test!(sync_log::stream(), "| Delete all index files.                     |");
        test!(sync_log::stream(), "| Start database:                             |");
        test!(sync_log::stream(), "|  * Including caching data files.            |");
        test!(sync_log::stream(), "| Fetch chunked data from previous session.   |");
        test!(sync_log::stream(), "| Gracefully shut down the database.          |");
        test!(sync_log::stream(), "+---------------------------------------------+");
        let mut db = match setup::start_db(
            db_root.clone(),
            Some(cfg.clone()),
            schms_input.clone(),
            Some(fmt!("./test_db_zone_container")),
            true,
            false,
        ) {
            Err(e) => return Err(delayed_error(e, error_delay)),
            Ok(db) => db,
        };

        match dbapi::fetch_chunked_data(
            &mut db,
            &key,
            &valvec,
            user,
            schms2,
        ) {
            Err(e) => return Err(delayed_error(e, error_delay)),
            _ => (),
        }

        test!(sync_log::stream(), "Listing files...");
        res!(db.api().list_files(wait));
        test!(sync_log::stream(), "Shutting db down...");
        res!(db.shutdown());
    }

    thread::sleep(Duration::from_secs(1));

    res!(corrupt_an_index_file(&zdirs));

    {
        test!(sync_log::stream(), "+---------------------------------------------+");
        test!(sync_log::stream(), "| NEW OZONE SESSION                           |");
        test!(sync_log::stream(), "| Corrupt a single byte of one index file.    |");
        test!(sync_log::stream(), "| Start database:                             |");
        test!(sync_log::stream(), "|  * Including caching index files.           |");
        test!(sync_log::stream(), "| Fetch chunked data from previous session.   |");
        test!(sync_log::stream(), "| Gracefully shut down the database.          |");
        test!(sync_log::stream(), "|                                             |");
        test!(sync_log::stream(), "| Note: We are expecting one error when an    |");
        test!(sync_log::stream(), "| igcbot discovers the corruption in the      |");
        test!(sync_log::stream(), "| index file and switches to indexing the     |");
        test!(sync_log::stream(), "| data file.                                  |");
        test!(sync_log::stream(), "+---------------------------------------------+");

        let mut db = match setup::start_db(
            db_root.clone(),
            Some(cfg.clone()),
            schms_input.clone(),
            Some(fmt!("./test_db_zone_container")),
            true,
            false,
        ) {
            Err(e) => return Err(delayed_error(e, error_delay)),
            Ok(db) => db,
        };

        test!(sync_log::stream(), "We expected an error during initial caching of an index file.");
        test!(sync_log::stream(), "This appears to have successfully resolved when the data file was instead cached.");

        test!(sync_log::stream(), "Listing files...");
        res!(db.api().list_files(wait));

        match dbapi::fetch_chunked_data(
            &mut db,
            &key,
            &valvec,
            user,
            schms2,
        ) {
            Err(e) => return Err(delayed_error(e, error_delay)),
            _ => (),
        }
        test!(sync_log::stream(), "Shutting db down...");
        res!(db.shutdown());
    }

    Ok(())
}
