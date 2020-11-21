use byteorder::{ByteOrder, LittleEndian};

#[derive(Clone)]
pub struct ConnectionCostMatrix {
    pub costs_data: Vec<u8>,
    pub backward_size: u32,
}

impl ConnectionCostMatrix {
    pub fn load(conn_data: &[u8]) -> ConnectionCostMatrix {
        let backward_size = LittleEndian::read_i16(&conn_data[2..4]);
        ConnectionCostMatrix {
            costs_data: conn_data[4..].to_vec(),
            backward_size: backward_size as u32,
        }
    }

    pub fn cost(&self, forward_id: u32, backward_id: u32) -> i32 {
        let cost_id = (backward_id + forward_id * self.backward_size) as usize;
        match self.costs_data.get(cost_id * 2) {
            Some(v) => {
                unsafe {
                    LittleEndian::read_i16(&[*v, *self.costs_data.get_unchecked(cost_id * 2 + 1)]) as i32
                }
            },
            None => 0,
        }
    }
}
