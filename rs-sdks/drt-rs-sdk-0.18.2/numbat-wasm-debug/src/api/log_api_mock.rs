use crate::TxContext;
use crate::TxLog;
use numbat_wasm::{api::LogApi, types::ArgBuffer};

/// Interface to only be used by code generated by the macros.
/// The smart contract code doesn't have access to these methods directly.
impl LogApi for TxContext {
    fn write_event_log(&self, topics_buffer: &ArgBuffer, data: &[u8]) {
        let arg_data_buffer = topics_buffer.arg_data();
        let arg_data_lengths = topics_buffer.arg_lengths();

        let mut current_index = 0;
        let mut topics = Vec::new();

        // we already processed the first data arg, so we skip it
        for arg_len in arg_data_lengths.iter() {
            let topic = arg_data_buffer[current_index..(current_index + arg_len)].to_vec();
            topics.push(topic);

            current_index += arg_len;
        }

        let mut tx_output_cell = self.tx_output_cell.borrow_mut();
        tx_output_cell.result.result_logs.push(TxLog {
            address: self.tx_input_box.to.clone(),
            endpoint: self.tx_input_box.func_name.clone(),
            topics,
            data: data.to_vec(),
        });
    }

    fn write_legacy_log(&self, topics: &[[u8; 32]], data: &[u8]) {
        let topics_vec = topics.iter().map(|array| array.to_vec()).collect();

        let mut tx_output_cell = self.tx_output_cell.borrow_mut();
        tx_output_cell.result.result_logs.push(TxLog {
            address: self.tx_input_box.to.clone(),
            endpoint: self.tx_input_box.func_name.clone(),
            topics: topics_vec,
            data: data.to_vec(),
        });
    }
}
