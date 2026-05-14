use crate::capture::stream::
    initialize_stream;

use crate::encoder::ffmpeg::
    initialize_ffmpeg;

use crate::encoder::hardware::
    initialize_hardware;

use crate::workers::pool::
    initialize_workers;

use crate::ipc::socket::
    initialize_socket;


pub fn initialize_runtime_services(){

    initialize_stream();

    initialize_ffmpeg();

    initialize_hardware();

    initialize_workers();

    initialize_socket();

}