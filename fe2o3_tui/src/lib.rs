//! A terminal user interface library and the Ironic Interactive Console implementation.
//! 
//! This crate provides abstractions for building terminal-based applications, with a focus on modal 
//! interaction patterns similar to Vi/Vim. The core library offers windowing, tabs, configurable styles,
//! scrollbars and both static and editable text views.
//!
//! The Ironic console demonstrates these capabilities in a general purpose terminal interface featuring:
//!
//! - Multiple independent windows that can be created, moved, resized and deleted
//! - Modal interface with navigation, editing and window management modes  
//! - Tab-based content organisation within windows
//! - Built-in support for files, logs, command shells and menus
//! - Comprehensive help system and status indicators
//! - Customisable styles and key bindings
//!
//! The interface is designed to be intuitive while providing powerful features for advanced users.
//! Windows can display various content types including text files, logs, file trees and command shells.
//! The modal design allows efficient keyboard-driven operation without overwhelming new users.
//!
#![forbid(unsafe_code)]
pub mod action;
pub mod cfg;
pub mod cmds;
pub mod draw;
pub mod event;
pub mod input;
//pub mod proc;
pub mod render;
pub mod repl;
pub mod style;
pub mod text;
pub mod window;