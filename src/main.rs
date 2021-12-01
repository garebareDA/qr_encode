use qr_encode::qr_bits_encode;
fn main () {
     match qr_bits_encode::qr_encode_mode("11231123") {
            Ok(qr_bits) => {
                 println!("{:?}", qr_bits.get_bits());
            },
            Err(e) => {
                 println!("{}", e);
            }
     }
}