// Ian Slane
// SOB prep
//  I planned on writing a bitcoin transaction parser.
// Overcomplicated and unfinished.
use std::collections::HashMap;

#[derive(Debug)]
struct Input {
    txid: String,
    vout: u32,
    scriptsig_len: u32,
    script_sig: String,
    sequence: u32,
}

#[derive(Debug)]
struct Output {
    amount: u32,
    scriptpubkey_len: u32,
    scriptpubkey: String,
}

fn main() {
    let raw_tx = "0100000001c997a5e56e104102fa209c6a852dd90660a20b2d9c352423edce25857fcd3704000000\
    004847304402204e45e16932b8af514961a1d3a1a25fdf3f4f7732e9d624c6c61548ab5fb8cd410220181522ec8eca07\
    de4860a4acdd12909d831cc56cbbac4622082221a8768d1d0901ffffffff0200ca9a3b00000000434104ae1a62fe09c5\
    f51b13905f07f06b99a2f7159b2225f374cd378d71302fa28414e7aab37397f554a7df5f142c21c1b7303b8a0626f1ba\
    ded5c72a704f7e6cd84cac00286bee0000000043410411db93e1dcdb8a016b49840f8c53bc1eb68a382e97b1482ecad7\
    b148a6909a5cb2e0eaddfb84ccf9744464f82e160bfa9b8b64f9d4c03f999b8643f656b412a3ac00000000";

    let mut parsed_tx: HashMap<String, String> = HashMap::new();

    let version = u32::from_str_radix(&raw_tx[..8], 16).unwrap();
    parsed_tx.insert("version".to_string(), version.to_string());

    let input_count = usize::from_str_radix(&raw_tx[8..10], 16).unwrap();
    parsed_tx.insert("input_count".to_string(), input_count.to_string());

    let mut inputs_end = 10;
    let mut inputs: Vec<Input> = Vec::new();
    for _ in 0..input_count {
        let txid = String::from(&raw_tx[inputs_end..inputs_end + 64]);
        inputs_end += 64;
        let vout = u32::from_str_radix(&raw_tx[inputs_end..inputs_end + 2], 16).unwrap();
        inputs_end += 2;
        let scriptsig_len = u32::from_str_radix(&raw_tx[inputs_end..inputs_end + 2], 16).unwrap();
        inputs_end += 2;
        let script_sig = String::from(&raw_tx[inputs_end..inputs_end + (scriptsig_len as usize * 2)]);
        inputs_end += scriptsig_len as usize * 2;
        let sequence = u32::from_str_radix(&raw_tx[inputs_end..inputs_end + 8], 16).unwrap();
        inputs_end += 8;
        let input = Input { txid, vout, scriptsig_len, script_sig, sequence };
        inputs.push(input);
    }
    parsed_tx.insert("inputs".to_string(), format!("{:?}", inputs));

    let output_count = usize::from_str_radix(&raw_tx[inputs_end..inputs_end + 2], 16).unwrap();
    parsed_tx.insert("output_count".to_string(), output_count.to_string());

    let mut outputs_end = inputs_end + 2;
    let mut outputs: Vec<Output> = Vec::new();
    for _ in 0..output_count {
        let amount = u32::from_str_radix(&raw_tx[outputs_end..outputs_end + 8], 16).unwrap();
        outputs_end += 8;
        let scriptpubkey_len = u32::from_str_radix(&raw_tx[outputs_end..outputs_end + 2], 16).unwrap();
        outputs_end += 2;
        let scriptpubkey = String::from(&raw_tx[outputs_end..outputs_end + (scriptpubkey_len as usize * 2)]);
        outputs_end += scriptpubkey_len as usize * 2;
        let output = Output { amount, scriptpubkey_len, scriptpubkey };
        outputs.push(output);
    }
    parsed_tx.insert("outputs".to_string(), format!("{:?}", outputs));

    let locktime_start = outputs_end;
    let locktime = u32::from_str_radix(&raw_tx[locktime_start..locktime_start + 8], 16).unwrap();


    parsed_tx.insert("locktime".to_string(), locktime.to_string());

    println!("{:?}", parsed_tx);
}
