use std::sync::Mutex;

use ts3plugin_sys::public_definitions::ClientProperties;

use crate::{TsApi};

#[derive(Debug)]
pub enum InitError {
	/// Initialisation failed, the plugin will be unloaded again
	Failure,
	/// Like `Failure`, but the client will not show a "failed to load" warning.
	/// This is a very special case and should only be used if a plugin displays
	/// a dialog (e.g. overlay) asking the user to disable the plugin again,
	/// avoiding the show another dialog by the client telling the user the
	/// plugin failed to load.
	/// For normal case, if a plugin really failed to load because of an error,
	/// the correct return value is `Failure`.
	FailureNoMessage
}

/// This trait that has to be implemented by a plugin. To enhance a library to a
/// working TeamSpeak plugin you have to call the macro [`create_plugin!`]
/// afterwards.
///
/// [`create_plugin!`]: ../macro.create_plugin.html
#[allow(unused_variables, unknown_lints, too_many_arguments)]
pub trait Plugin: 'static + Send {
	// ************************* Configuration methods *************************
	/// The name of the plugin as displayed in TeamSpeak.
	fn name() -> String where Self: Sized;
	/// The version of the plugin as displayed in TeamSpeak.
	fn version() -> String where Self: Sized;
	/// The author of the plugin as displayed in TeamSpeak.
	fn author() -> String where Self: Sized;
	/// The description of the plugin as displayed in TeamSpeak.
	fn description() -> String where Self: Sized;
	/// The command prefix that can be used by users in the chat, defaults to `None`.
	fn command() -> Option<String> where Self: Sized { None }
	/// If the plugin offers the possibility to be configured, defaults to
	/// [`ConfigureOffer::No`].
	///
	/// [`ConfigureOffer::No`]: ../ts3plugin_sys/plugin_definitions/enum.ConfigureOffer.html
	fn configurable() -> ::ConfigureOffer where Self: Sized { ::ConfigureOffer::No }
	/// If the plugin should be loaded by default or only if activated manually,
	/// defaults to `false`.
	fn autoload() -> bool where Self: Sized { false }

	// *************************** Required methods ****************************
	/// Called when the plugin is loaded by TeamSpeak.
	fn new(api: &mut ::TsApi) -> Result<Box<Self>, InitError> where Self: Sized;

	// *************************** Optional methods ****************************
	/// If the connection status changes.
	/// If `status = ConnectStatus::Connecting`, the connection_id is not yet
	/// registered in the [`TsApi`].
	///
	/// [`TsApi`]: ../struct.TsApi.html
	fn connect_status_change(&mut self, api: &mut ::TsApi, server_id: ::ServerId, status:
		::ConnectStatus, error: ::Error) {}

	/// Called if a server is stopped. The server sends also a stop message.
	fn server_stop(&mut self, api: &mut ::TsApi, server_id: ::ServerId, message: String) {}

	fn configure(&mut self, api: &mut ::TsApi) {}

	/// Called if a server error occurs.
	/// Return `false` if the TeamSpeak client should handle the error normally or
	/// `true` if the client should ignore the error.
	fn server_error(&mut self, api: &mut ::TsApi, server_id: ::ServerId,
		error: ::Error, message: String, return_code: String,
		extra_message: String) -> bool { false }

	/// Called if someone edited the server.
	fn server_edited(&mut self, api: &mut ::TsApi, server_id: ::ServerId,
		invoker: Option<::Invoker>) {}

	/// Called when the user requests the server info by middle-clicking on the server.
	fn server_connection_info(&mut self, api: &mut ::TsApi, server_id: ::ServerId) {}

	fn connection_info(&mut self, api: &mut ::TsApi, server_id: ::ServerId,
		connection_id: ::ConnectionId) {}

	fn connection_updated(&mut self, api: &mut ::TsApi, server_id: ::ServerId,
		connection_id: ::ConnectionId, old_connection: Option<::Connection>, invoker: ::Invoker) {}

	/// If the plugin was informed about a new connection. If appeared is true, the connection
	/// was previously not known to the plugin, if appeared is false, the connection left
	/// the view of connection.
	fn connection_announced(&mut self, api: &mut ::TsApi, server_id: ::ServerId,
		connection_id: ::ConnectionId, appeared: bool) {}

	/// Called, if a connection connects to the server. This is also called for our own
	/// connection.
	fn connection_changed(&mut self, api: &mut ::TsApi, server_id: ::ServerId,
		connection_id: ::ConnectionId, connected: bool, message: String) {}

	/// Called if a connection switched the channel.
	fn connection_move(&mut self, api: &mut ::TsApi, server_id: ::ServerId,
		connection_id: ::ConnectionId, old_channel_id: ::ChannelId,
		new_channel_id: ::ChannelId, visibility: ::Visibility) {}

	/// Called if a connection was moved by another connection.
	fn connection_moved(&mut self, api: &mut ::TsApi, server_id: ::ServerId,
		connection_id: ::ConnectionId, old_channel_id: ::ChannelId,
		new_channel_id: ::ChannelId, visibility: ::Visibility, invoker: ::Invoker) {}

	/// Called when a connection times out.
	fn connection_timeout(&mut self, api: &mut ::TsApi, server_id: ::ServerId,
		connection_id: ::ConnectionId) {}

	/// Called if a channel is announced to the client.
	/// This will be called for each channel when connecting to a server.
	fn channel_announced(&mut self, api: &mut ::TsApi, server_id: ::ServerId,
		channel_id: ::ChannelId) {}

	/// Called if the channel description was changed.
	fn channel_description_updated(&mut self, api: &mut ::TsApi, server_id: ::ServerId,
		channel_id: ::ChannelId) {}

	/// Called if the channel data are updated and available.
	/// This happens e.g. when the user clicked on the channel for the first time.
	fn channel_updated(&mut self, api: &mut ::TsApi, server_id: ::ServerId,
		channel_id: ::ChannelId, old_channel: Option<::Channel>) {}

	/// Called if a channel was created.
	/// The invoker is `None` if the server created the channel.
	fn channel_created(&mut self, api: &mut ::TsApi, server_id: ::ServerId,
		channel_id: ::ChannelId, invoker: Option<::Invoker>) {}

	/// Called if a channel was deleted.
	/// The invoker is `None` if the server deleted the channel.
	fn channel_deleted(&mut self, api: &mut ::TsApi, server_id: ::ServerId,
		channel_id: ::ChannelId, invoker: Option<::Invoker>) {}

	/// Called if a channel was edited.
	fn channel_edited(&mut self, api: &mut ::TsApi, server_id: ::ServerId,
		channel_id: ::ChannelId, old_channel: Option<::Channel>, invoker: ::Invoker) {}

	/// Called if the channel password was updated.
	fn channel_password_updated(&mut self, api: &mut ::TsApi, server_id: ::ServerId,
		channel_id: ::ChannelId) {}

	/// The current parent id of the channel is the old one, the new
	/// parent id is given as a parameter.
	fn channel_moved(&mut self, api: &mut ::TsApi, server_id: ::ServerId,
		channel_id: ::ChannelId, new_parent_channel_id: ::ChannelId,
		invoker: Option<::Invoker>) {}

	/// A message was received. `ignored` describes, if the friend and fool system
	/// of TeamSpeak ignored the message.
	/// Return `false` if the TeamSpeak client should handle the message normally or
	/// `true` if the client should ignore the message.
	fn message(&mut self, api: &mut ::TsApi, server_id: ::ServerId, invoker: ::Invoker,
		target: ::MessageReceiver, message: String, ignored: bool) -> bool { false }

	/// A user poked us. `ignored` describes, if the friend and fool system
	/// of TeamSpeak ignored the message.
	/// Return `false` if the TeamSpeak client should handle the poke normally or
	/// `true` if the client should ignore the poke.
	fn poke(&mut self, api: &mut ::TsApi, server_id: ::ServerId, invoker: ::Invoker,
		message: String, ignored: bool) -> bool { false }

	fn channel_kick(&mut self, api: &mut ::TsApi, server_id: ::ServerId,
		connection_id: ::ConnectionId, old_channel_id: ::ChannelId, new_channel_id: ::ChannelId,
		visibility: ::Visibility, invoker: ::Invoker, message: String) {}

	fn server_kick(&mut self, api: &mut ::TsApi, server_id: ::ServerId,
		connection_id: ::ConnectionId, invoker: ::Invoker, message: String) {}

	fn server_ban(&mut self, api: &mut ::TsApi, server_id: ::ServerId,
		connection_id: ::ConnectionId, invoker: ::Invoker, message: String, time: u64) {}

	/// The old values of `talking` and `whispering` are available from the connection.
	/// They will be updated after this functions returned.
	fn talking_changed(&mut self, api: &mut ::TsApi, server_id: ::ServerId,
		connection_id: ::ConnectionId, talking: ::TalkStatus, whispering: bool) {}

	fn ev_3drollof_calculation(&mut self, api: &mut ::TsApi, server_id: ::ServerId,
		client_id: ::ConnectionId, distance: f32, volume: &mut f32){ }

	/// Called if the avatar of a client is updated.
	/// This also happens when the avatar is discovered for the first time.
	/// The avatar information are only fetched if requested, e.g. if the
	/// user clicks on a connection.
	fn avatar_changed(&mut self, api: &mut ::TsApi, server_id: ::ServerId,
		connection_id: ::ConnectionId, path: Option<String>) {}

	fn self_variable_update(&mut self, api: &mut ::TsApi, server_id: ::ServerId, 
		flag: ClientProperties, old_value: String, new_value: String) {}

	/// Called if a channel group is assigned to a connection.
	fn connection_channel_group_changed(&mut self, api: &mut ::TsApi,
		server_id: ::ServerId, connection_id: ::ConnectionId, channel_group_id: ::ChannelGroupId,
		channel_id: ::ChannelId, invoker: ::Invoker) {}

	/// Called if a server group is added to a connection.
	fn connection_server_group_added(&mut self, api: &mut ::TsApi, server_id: ::ServerId,
		connection: ::Invoker, server_group_id: ::ServerGroupId, invoker: ::Invoker) {}

	/// Called if a server group is removed from a connection.
	fn connection_server_group_removed(&mut self, api: &mut ::TsApi, server_id: ::ServerId,
		connection: ::Invoker, server_group_id: ::ServerGroupId, invoker: ::Invoker) {}

	/// Called when a voice packet from a client was received.
	///
	/// From the TeamSpeak documentation:
	/// The following event is called when a voice packet from a client (not own
	/// client) is decoded and about to be played over your sound device, but
	/// before it is 3D positioned and mixed with other sounds. You can use this
	/// function to alter the voice data (for example when you want to do
	/// effects on it) or to simply get voice data. The TeamSpeak client uses
	/// this function to record sessions.
	///
	/// The voice data is available as 16 bit with 48 KHz. The channels are packed
	/// (interleaved).
	/// The callbacks with audio data are called from another thread than the
	/// other functions.
	fn playback_voice_data(&mut self, api: &mut ::TsApi, server_id: ::ServerId,
		connection_id: ::ConnectionId, samples: &mut [i16], channels: i32) {}

	/// Called when a voice packet from a client was positioned.
	///
	/// From the TeamSpeak documentation:
	/// The following event is called when a voice packet from a client (not own
	/// client) is decoded and 3D positioned and about to be played over your
	/// sound device, but before it is mixed with other sounds. You can use this
	/// function to alter/get the voice data after 3D positioning.
	///
	/// The voice data is available as 16 bit with 48 KHz. The channels are packed
	/// (interleaved).
	/// The callbacks with audio data are called from another thread than the
	/// other functions.
	fn post_process_voice_data(&mut self, api: &mut ::TsApi, server_id: ::ServerId,
		connection_id: ::ConnectionId, samples: &mut [i16], channels: i32,
		channel_speaker_array: &[::Speaker], channel_fill_mask: &mut u32) {}

	/// Called when all voice data were mixed.
	///
	/// From the TeamSpeak documentation:
	/// The following event is called when all sounds that are about to be
	/// played back for this server connection are mixed. This is the last
	/// chance to alter/get sound.
	///
	/// The voice data is available as 16 bit with 48 KHz. The channels are packed
	/// (interleaved).
	/// The callbacks with audio data are called from another thread than the
	/// other functions.
	fn mixed_playback_voice_data(&mut self, api: &mut ::TsApi, server_id: ::ServerId,
		samples: &mut [i16], channels: i32, channel_speaker_array: &[::Speaker],
		channel_fill_mask: &mut u32) {}

	/// The recorded sound from the current capture device.
	///
	/// `send` is set if the audio data will be send to the server. This attribute
	/// can be changed in this callback.
	/// The return value of this function describes if the sound data was altered.
	/// Return `true` if the sound was changed and `false` otherwise.
	/// The callbacks with audio data are called from another thread than the
	/// other functions.
	fn captured_voice_data(&mut self, api: &mut ::TsApi, server_id: ::ServerId,
		samples: &mut [i16], channels: i32, send: &mut bool) -> bool { false }

	/// Return `false` if the TeamSpeak client should handle the error normally or
	/// `true` if the client should ignore the error.
	fn permission_error(&mut self, api: &mut ::TsApi, server_id: ::ServerId,
		permission_id: ::PermissionId, error: ::Error, message: String,
		return_code: String) -> bool { false }

	/// Called when a message from another plugin is received.
	///
	/// Messages can be sent with [`Server::send_plugin_message`].
	/// The message is called `PluginCommand` by TeamSpeak.
	///
	/// [`Server::send_plugin_message`]: ../struct.Server.html#method.send_plugin_message
	fn plugin_message(&mut self, api: &mut ::TsApi, server_id: ::ServerId,
		plugin: String, message: String) { }

	/// Called when the user enters a command in the chat box.
	///
	/// Commands that are prefixed with the string, which is specified in
	/// [`Plugin::command`], are redirected to this function.
	/// The command prefix is not contained in the given argument.
	///
	/// Return `true` if this function handled the command or `false` if not.
	///
	/// [`Plugin::command`]: #method.command
	fn process_command(&mut self, api: &mut ::TsApi, server_id: ::ServerId,
		command: String) -> bool { false }

	/// Called if the plugin is getting disabled (either by the user or if
	/// TeamSpeak is exiting).
	fn shutdown(&mut self, api: &mut ::TsApi) {}
}

/// Save the `CString`s that are returned from the TeamSpeak API.
/// We don't want to return invalid pointers.
#[doc(hidden)]
pub struct CreatePluginData {
	pub name: Option<::std::ffi::CString>,
	pub version: Option<::std::ffi::CString>,
	pub author: Option<::std::ffi::CString>,
	pub description: Option<::std::ffi::CString>,
	pub command: Option<Option<::std::ffi::CString>>,
}

lazy_static! {
	#[doc(hidden)]
	pub static ref CREATE_PLUGIN_DATA: Mutex<CreatePluginData> =
		Mutex::new(CreatePluginData {
			name: None,
			version: None,
			author: None,
			description: None,
			command: None,
		});
}

/// Create a plugin.
///
/// This macro has to be called once per library to create the
/// function interface that is used by TeamSpeak. The argument is the struct
/// which implements the [`Plugin`] trait.
///
/// # Examples
///
/// ```ignore
/// create_plugin!(MyTsPlugin);
/// ```
///
/// [`Plugin`]: plugin/trait.Plugin.html
#[macro_export]
macro_rules! create_plugin {
	($typename: ident) => {
		/// Initialise the plugin and return the error status.
		#[no_mangle]
		#[doc(hidden)]
		pub unsafe extern "C" fn ts3plugin_init() -> std::os::raw::c_int {
			match $crate::ts3interface::private_init::<$typename>() {
				Ok(_) => 0,
				Err($crate::InitError::Failure) => 1,
				Err($crate::InitError::FailureNoMessage) => -2,
			}
		}

		/// Unique name identifying this plugin.
		/// The result of this function has to be a null-terminated static string.
		/// Can be called before init.
		#[no_mangle]
		#[doc(hidden)]
		pub extern "C" fn ts3plugin_name() -> *const std::os::raw::c_char {
			let mut data = CREATE_PLUGIN_DATA.lock().unwrap();
			if data.name.is_none() {
				let s = $typename::name();
				let s = ::std::ffi::CString::new(s).expect("String contains nul character");
				data.name = Some(s);
			}
			data.name.as_ref().unwrap().as_ptr()
		}

		/// The version of the plugin.
		/// Can be called before init.
		#[no_mangle]
		#[doc(hidden)]
		pub extern "C" fn ts3plugin_version() -> *const std::os::raw::c_char {
			let mut data = CREATE_PLUGIN_DATA.lock().unwrap();
			if data.version.is_none() {
				let s = $typename::version();
				let s = ::std::ffi::CString::new(s).expect("String contains nul character");
				data.version = Some(s);
			}
			data.version.as_ref().unwrap().as_ptr()
		}

		/// The author of the plugin.
		/// Can be called before init.
		#[no_mangle]
		#[doc(hidden)]
		pub extern "C" fn ts3plugin_author() -> *const std::os::raw::c_char {
			let mut data = CREATE_PLUGIN_DATA.lock().unwrap();
			if data.author.is_none() {
				let s = $typename::author();
				let s = ::std::ffi::CString::new(s).expect("String contains nul character");
				data.author = Some(s);
			}
			data.author.as_ref().unwrap().as_ptr()
		}

		/// The desription of the plugin.
		/// Can be called before init.
		#[no_mangle]
		#[doc(hidden)]
		pub extern "C" fn ts3plugin_description() -> *const std::os::raw::c_char {
			let mut data = CREATE_PLUGIN_DATA.lock().unwrap();
			if data.description.is_none() {
				let s = $typename::description();
				let s = ::std::ffi::CString::new(s).expect("String contains nul character");
				data.description = Some(s);
			}
			data.description.as_ref().unwrap().as_ptr()
		}

		/// If the plugin offers the possibility to be configured by the user.
		/// Can be called before init.
		#[allow(non_snake_case)]
		#[no_mangle]
		#[doc(hidden)]
		pub extern "C" fn ts3plugin_commandKeyword() -> *const std::os::raw::c_char {
			let mut data = CREATE_PLUGIN_DATA.lock().unwrap();
			if data.command.is_none() {
				data.command = Some(if let Some(s) = $typename::command() {
					let s = ::std::ffi::CString::new(s).expect("String contains nul character");
					Some(s)
				} else {
					None
				})
			}
			if let &Some(ref s) = data.command.as_ref().unwrap() {
				s.as_ptr()
			} else {
				std::ptr::null()
			}
		}

		/// If the plugin offers the possibility to be configured by the user.
		/// Can be called before init.
		#[allow(non_snake_case)]
		#[no_mangle]
		#[doc(hidden)]
		pub extern "C" fn ts3plugin_offersConfigure() -> std::os::raw::c_int {
			$typename::configurable() as std::os::raw::c_int
		}

		/// If the plugin should be loaded automatically.
		/// Can be called before init.
		#[allow(non_snake_case)]
		#[no_mangle]
		#[doc(hidden)]
		pub extern "C" fn ts3plugin_requestAutoload() -> std::os::raw::c_int {
			if $typename::autoload() {
				1
			} else {
				0
			}
		}
	};
}
