use crate::{
    prelude::*,
    base::{
        constant,
        id::{
            Bid,
            OzoneBotId,
        },
    },
    bots::{
        base::{
            bot::{
                BotInitArgs,
                OzoneBot,
            },
            handles::Handle,
        },
        bot_super::Supervisor,
    },
    comm::{
        channels::BotChannels,
        msg::OzoneMsg,
        response::Responder,
    },
    data::{
        core::{
            RestSchemes,
            RestSchemesInput,
        },
    },
    file::core::find_files,
};

use oxedize_fe2o3_bot::Bot;
use oxedize_fe2o3_core::{
    channels::{
        simplex,
        Simplex,
        Recv,
    },
    path::NormalPath,
    rand::RanDef,
    thread::thread_channel,
};
use oxedize_fe2o3_jdat::{
    prelude::*,
    cfg::Config,
    file::JdatMapFile,
    id::NumIdDat,
};
use oxedize_fe2o3_namex::id::{
    InNamex,
    NamexId,
};
use oxedize_fe2o3_text::string::Stringer;

use std::{
    fs,
    path::{
        Path,
        PathBuf,
    },
    sync::{
        Arc,
        RwLock,
    },
    thread,
    time::Duration,
};

use crossbeam_utils::sync::WaitGroup;


/// The main Ozone database struct.
///
/// # Storage specifications
/// The user can change the various data transformation schemes in three ways.
/// ## Invocation
/// Upon invocation, rest schemes conforming to the traits `oxedize_fe2o3_iop_hash::csum::Checksummer`,
/// `oxedize_fe2o3_iop_hash::api::Hasher`, `oxedize_fe2o3_iop_crypto::enc::Encrypter` and
/// `oxedize_fe2o3_iop_crypto::sign::Signer` can be given to the `O3db` instance.  When a scheme is not
/// provided, a hardwired default is set.
/// ## Configuration
/// Default schemes can be overridden at invocation or upon any subsequent configuration file
/// changes.  Schemes are limited to those provided in `oxedize_fe2o3_hash` and `oxedize_fe2o3_crypto`.
/// ## Per value basis
/// Finally, schemes for storage of data at rest can be set explicitely for any key-value pair,
/// overriding invocation or default schemes.
///
/// # Directory layout
/// The example below shows a database that has been used with 3 and 5 zones.  The database is
/// invoked with the absolute path to the db_root, and an optional `OzoneConfig` which is used if
/// a configuration file is not found.
///
/// ```ignore
///
/// /../my_o3db                         <- db_root with db_name, aka db_container
/// ├── config.jdat
/// ├── 003_zone                        <- zone_root
/// │   ├── zone_001                    <- zone_dir
/// │   │   ├── 000_000_001.dat
/// │   │   ├── 000_000_001.ind
/// │   │   ├── 000_000_002.dat
/// │   │   └── 000_000_002.ind
/// │   ├── zone_002                    <- zone_dir
/// │   │   ├── 000_000_001.dat
/// │   │   ├── 000_000_001.ind
/// │   │   ├── 000_000_002.dat
/// │   │   └── 000_000_002.ind
/// │   └── zone_003                    <- zone_dir
/// │       ├── 000_000_001.dat
/// │       ├── 000_000_001.ind
/// │       ├── 000_000_002.dat
/// │       └── 000_000_002.ind
/// └── 005_zone                        <- zone_root
///     ├── zone_002                    <- zone_dir
///     │   ├── 000_000_001.dat
///     │   └── 000_000_001.ind
///     ├── zone_003                    <- zone_dir
///     │   ├── 000_000_001.dat
///     │   └── 000_000_001.ind
///     ├── zone_004                    <- zone_dir
///     │   ├── 000_000_001.dat
///     │   └── 000_000_001.ind
///     └── zone_005                    <- zone_dir
///         ├── 000_000_001.dat
///         └── 000_000_001.ind
///
/// a_zone_container_dir                <- zone_container
/// └── 005_zone                        <- zone_root
///     └─── zone_001                   <- zone_dir
///         ├── 000_000_001.dat
///         └── 000_000_001.ind
/// ```
#[derive(Clone, Debug)]
pub struct O3db<
    const UIDL: usize,        // User identifier byte length.
    UID:    NumIdDat<UIDL>,   // User identifier.            
    ENC:    Encrypter,        // Symmetric encryption of data at rest.
    KH:     Hasher,           // Hashes database keys.
	PR:     Hasher,           // Pseudo-randomiser hash to distribute cache data.
    CS:     Checksummer,      // Checks integrity of data at rest.
>{
    db_root:    PathBuf,
    chan_inbox: Simplex<OzoneMsg<UIDL, UID, ENC, KH>>,
    api:        OzoneApi<UIDL, UID, ENC, KH, PR, CS>,
    wg_end:     WaitGroup,
}

impl<
    const UIDL: usize,
    UID:    NumIdDat<UIDL> + 'static,
    ENC:    Encrypter + 'static,
    KH:     Hasher + 'static,
	PR:     Hasher + 'static,
    CS:     Checksummer + 'static,
>
    O3db<UIDL, UID, ENC, KH, PR, CS>
{
    /// Create a new Ozone database instance.  Some validation is performed, but the database is
    /// not properly activated until `O3db::start` is called.
    pub fn new<P: Into<PathBuf>>(
        db_root:        P,
        cfg_opt:        Option<OzoneConfig>,
        schms_input:    RestSchemesInput<ENC, KH, PR, CS>,
        _uid_template:  UID,
    )
        -> Outcome<Self>
    {
        // Check constants.
        res!(OzoneConfig::check_constants());

        let db_root = db_root.into();
        if !db_root.exists() {
            warn!("Ozone database root directory {:?} does not exist, attempting to create...",
                db_root);
            res!(fs::create_dir_all(&db_root));
            info!("{:?} created.", db_root);
        }

        let cfg_path = OzoneConfig::config_path(&db_root);
        let mut cfg = if cfg_path.is_file() {
            res!(<OzoneConfig as JdatMapFile>::load(&cfg_path))
        } else {
            match cfg_opt {
                Some(cfg) => {
                    res!(cfg.save(&cfg_path, "  ", true));
                    warn!(
                        "Configuration file {:?} saved, using default configuration provided.",
                        cfg_path,
                    );
                    cfg
                }
                None => return Err(err!(errmsg!(
                    "You must supply an OzoneConfig.",
                ), Input, Missing)),
            }
        };

        // Check configuration.
        res!(cfg.check_and_fix());

        // File system.
        let zone_root = cfg.zone_root(&db_root);
        res!(fs::create_dir_all(&zone_root));

        let schms = RestSchemes::from(schms_input);

        let chans = BotChannels::new(&cfg); // This is the original from which all derive.
        let ozid = OzoneBotId::Master(Bid::randef());
        
        let api = OzoneApi::new(
            ozid,
            db_root.clone(),
            cfg,
            chans,
            schms,
        );

        Ok(Self {
            db_root,
            chan_inbox: simplex(),
            api,
            wg_end: WaitGroup::default(),
        })
    }

    pub fn db_root(&self)       -> &Path { &self.db_root }
    pub fn api(&self)           -> &OzoneApi<UIDL, UID, ENC, KH, PR, CS> { &self.api }
    pub fn api_mut(&mut self)   -> &mut OzoneApi<UIDL, UID, ENC, KH, PR, CS> { &mut self.api }

    /// Thread-safe mutable sharing of the API.
    pub fn share_api(self) -> Arc<RwLock<OzoneApi<UIDL, UID, ENC, KH, PR, CS>>> {
        Arc::new(RwLock::new(self.api))
    }

    pub fn updated_api(&mut self) -> Outcome<&mut OzoneApi<UIDL, UID, ENC, KH, PR, CS>> {
        res!(self.update());
        Ok(&mut self.api)
    }

    // Convenience.
    pub fn ozid(&self)      -> &OzoneBotId                      { &self.api.ozid }
    pub fn cfg(&self)       -> &OzoneConfig                     { &self.api.cfg }
    pub fn chans(&self)     -> &BotChannels<UIDL, UID, ENC, KH>          { &self.api.chans }
    pub fn schemes(&self)   -> &RestSchemes<ENC, KH, PR, CS>    { &self.api.schms }
    pub fn responder(&self) -> Responder<UIDL, UID, ENC, KH> { Responder::new(Some(&self.ozid())) }
    pub fn no_responder()   -> Responder<UIDL, UID, ENC, KH> { Responder::none(None) }

    pub fn update(&mut self) -> Outcome<()> {
        loop { // loop to ensure we get the latest BotChannels
            match self.chan_inbox.try_recv() {
                Recv::Empty => break,
                Recv::Result(Err(e)) => {
                    return Err(e);
                },
                Recv::Result(Ok(msg)) => match msg {
                    OzoneMsg::Channels(chans, resp) => {
                        self.api.chans = chans;
                        res!(resp.send(
                            OzoneMsg::ChannelsReceived(self.ozid().clone()))
                        );
                    },
                    OzoneMsg::Config(cfg) => {
                        self.api.cfg = cfg;
                    },
                    _ => {
                        return Err(err!(errmsg!(
                            "{}: Unrecognised channel update message: {:?}.",
                            self.ozid(), msg,
                        ), Invalid, Input, Channel));
                    },
                }
            }
        }
        Ok(())
    }

    /// Start the Ozone database.
    pub fn start(&mut self) -> Outcome<()> {

        for line in constant::SPLASH.split("\n") {
            info!("{}", line);
        }
        for line in Stringer::new(fmt!("{:?}", self.schemes())).to_lines("  ") {
            info!("{}", line);
        }
        // Write config to a file now that we have a directory structure.
        res!(self.cfg().write_config_file(self.db_root()));

        // Create and start the supervisor.
        let (semaphore, sentinel) = thread_channel();
        let api = OzoneApi::new(
            OzoneBotId::Supervisor(Bid::randef()),
            self.db_root.clone(),
            self.cfg().clone(),
            self.chans().clone(),
            self.schemes().clone(),
        );
        let args = BotInitArgs {
            // Bot
            sem:        semaphore,
            // Comms
            chan_in:    self.chans().sup().clone(),
            // API
            api,
        };
        let mut sup = Supervisor::new(
            args,
            self.chan_inbox.clone(),
        );
        res!(sup.init()); // Starts all the other bots.
        self.wg_end = sup.handles().wait_end_ref().clone();
        let wg_end = self.wg_end.clone();
        let builder = thread::Builder::new()
            .name(sup.ozid().to_string())
            .stack_size(constant::STACK_SIZE);
        Handle::new(
            Some(sup.ozid().clone()),
            res!(builder.spawn(move || {
                sup.go();
                drop(wg_end);
            })),
            sentinel,
            Some(self.chans().sup().clone()),
        );
        thread::sleep(Duration::from_secs(1));

        //// Initialise users.
        //res!(self.init_users());

        info!("Database initialisation and activation complete.");
        
        Ok(())
    }

    /// Find all data and index files of the existing database.
    pub fn find_all_data_files(&self) -> Outcome<Vec<PathBuf>> {

        let mut found_files = Vec::new();

        let cur_dir = res!(std::env::current_dir());
        info!("The current directory is {}", cur_dir.display());
        
        let db_root = &self.db_root;
        
        info!("Searching for all data and index files in {:?}", db_root);

        if db_root.exists() && db_root.is_dir() {                           
            let files = res!(find_files(&db_root));
            for file in files {
                found_files.push(file);
            }
        }

        for (zind_dat, zone_dat) in self.cfg().zone_overrides() { 
            if let Ok(Some(Dat::Str(dir))) = zone_dat.map_get(&dat!("dir")) {
                let dir = db_root.join(dir).normalise();
                info!("Searching for all data and index files in zone {:?} override {:?}",
                    zind_dat, dir);
                let files = res!(find_files(&dir));
                for file in files {
                    found_files.push(file);
                }
            }
        }

        Ok(found_files)
    }

    /// Gracefully shut down the database, including the supervisor. 
    pub fn shutdown(mut self) -> Outcome<()> {
        res!(self.update());
        let self_id = self.ozid();
        let resp = self.responder();
        if let Err(e) = self.chans().sup().send(
            OzoneMsg::Shutdown(self_id.clone(), resp.clone())
        ) {
            return Err(err!(e, errmsg!(
                "{}: Cannot send shutdown request to supervisor.", self_id,
            ), Channel, Write));
        }
        warn!("Shutdown: Waiting for response from supervisor...");
        match res!(resp.recv_timeout(constant::USER_REQUEST_TIMEOUT)) {
            OzoneMsg::Error(e) => return Err(err!(e, errmsg!(
                "{}: The supervisor had a problem during shutdown.", self_id,
            ))),
            OzoneMsg::Ok => (),
            msg => return Err(err!(errmsg!(
                "{}: Unexpected response from supervisor during shutdown: {:?}", self_id, msg,
            ))),
        }
        warn!("Shutdown: Succesfully completed by supervisor, waiting for final \
            verification of termination of all threads...");
        self.wg_end.wait();
        warn!("Shutdown: Verified.");
        Ok(())
    }

}

impl<
    const UIDL: usize,
    UID:    NumIdDat<UIDL> + 'static,
    ENC:    Encrypter + 'static,
    KH:     Hasher + 'static,
	PR:     Hasher + 'static,
    CS:     Checksummer + 'static,
>
    InNamex for O3db<UIDL, UID, ENC, KH, PR, CS>
{
    fn name_id(&self) -> Outcome<NamexId> {
        NamexId::try_from(constant::NAMEX_ID)
    }
}