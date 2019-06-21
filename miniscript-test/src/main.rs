extern crate bitcoin;
extern crate miniscript;
extern crate secp256k1;


fn roundtrip(tree: &miniscript::Miniscript<bitcoin::PublicKey>, s: &str) {
    println!("{}", tree);
    let ser = tree.encode();
    assert_eq!(ser.len(), tree.script_size());
    println!("{}", ser);
    assert_eq!(ser.to_string(), s);
    let deser = miniscript::Miniscript::parse(&ser).expect("deserialize result of serialize");
    assert_eq!(tree, &deser);
}

fn pubkeys(n: usize) -> Vec<bitcoin::PublicKey> {
    let mut ret = Vec::with_capacity(n);
    let secp = secp256k1::Secp256k1::new();
    let mut sk = [0; 32];
    for i in 1..n + 1 {
        sk[0] = i as u8;
        sk[1] = (i >> 8) as u8;
        sk[2] = (i >> 16) as u8;

        let pk = bitcoin::PublicKey {
            key: secp256k1::PublicKey::from_secret_key(
                &secp,
                &secp256k1::SecretKey::from_slice(&sk[..]).expect("secret key"),
            ),
            compressed: true,
        };
        ret.push(pk);
    }
    ret
}

fn main() {
    let keys = pubkeys(5);
    roundtrip(
        &miniscript::Miniscript(miniscript::AstElem::Pk(keys[0].clone())),
        "Script(OP_PUSHBYTES_33 028c28a97bf8298bc0d23d8c749452a32e694b65e30a9472a3954ab30fe5324caa OP_CHECKSIG)",
    );
}
