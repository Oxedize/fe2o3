use crate::{
    http::{
        msg::HttpMessage,
        loc::HttpLocator,
    },
};

use oxedize_fe2o3_core::prelude::*;
use oxedize_fe2o3_iop_crypto::enc::Encrypter;
use oxedize_fe2o3_iop_db::api::Database;
use oxedize_fe2o3_iop_hash::api::Hasher;
use oxedize_fe2o3_jdat::id::NumIdDat;

use std::{
    sync::{
        Arc,
        RwLock,
    },
};


pub trait WebHandler:
    Clone
    + std::fmt::Debug
    + Send
    + Sync
{
    fn handle_get<
        const SIDL: usize,
        const UIDL: usize,
        SID:    NumIdDat<SIDL> + 'static,
        UID:    NumIdDat<UIDL> + 'static,
        ENC:    Encrypter,
        KH:     Hasher,
        DB:     Database<UIDL, UID, ENC, KH>,
    >(
        &self,
        loc:        HttpLocator,
        response:   Option<HttpMessage>,
        body:       Vec<u8>,
        db:         Option<(Arc<RwLock<DB>>, UID)>,
        sid_opt:    &Option<SID>,
        id:         &String, 
    )
        -> impl std::future::Future<Output = Outcome<Option<HttpMessage>>> + Send;
    
    fn handle_post<
        const SIDL: usize,
        const UIDL: usize,
        SID:    NumIdDat<SIDL> + 'static,
        UID:    NumIdDat<UIDL> + 'static,
        ENC:    Encrypter,
        KH:     Hasher,
        DB:     Database<UIDL, UID, ENC, KH>,
    >(
        &self,
        loc:        HttpLocator,
        response:   Option<HttpMessage>,
        body:       Vec<u8>,
        db:         Option<(Arc<RwLock<DB>>, UID)>,
        sid_opt:    &Option<SID>,
        id:         &String, 
    )
        -> impl std::future::Future<Output = Outcome<Option<HttpMessage>>> + Send;
}