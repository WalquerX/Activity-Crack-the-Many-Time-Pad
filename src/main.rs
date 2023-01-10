fn cv(x: &str) -> Vec<u8> {
    x.as_bytes()
        .chunks(2)
        .into_iter()
        .map(|ac| {
            let con: Vec<u8> = ac
                .iter()
                .map(|a| match *a {
                    (b'0'..=b'9') => a - b'0',
                    (b'A'..=b'F') => a - b'A' + 10,
                    (b'a'..=b'f') => a - b'a' + 10,
                    _ => panic!("error"),
                })
                .collect();
            con[1] | con[0] << 4
        })
        .collect()
}

fn xor(v1: &Vec<u8>, v2: &Vec<u8>) -> Vec<u8> {
    v1.iter().zip(v2.iter()).map(|(s1, s2)| s1 ^ s2).collect()
}

fn main() {
    let x2 = "FF00AAF0";

    //println!("{:?}", cv(x2));
    let c = hex::decode(x2);

    let s0: &str = "160111433b00035f536110435a380402561240555c526e1c0e431300091e4f04451d1d490d1c49010d000a0a4510111100000d434202081f0755034f13031600030d0204040e";
    let s1: &str = "050602061d07035f4e3553501400004c1e4f1f01451359540c5804110c1c47560a1415491b06454f0e45040816431b144f0f4900450d1501094c1b16550f0b4e151e03031b450b4e020c1a124f020a0a4d09071f16003a0e5011114501494e16551049021011114c291236520108541801174b03411e1d124554284e141a0a1804045241190d543c00075453020a044e134f540a174f1d080444084e01491a090b0a1b4103570740";
    let s2: &str = "000000000000001a49320017071704185941034504524b1b1d40500a0352441f021b0708034e4d0008451c40450101064f071d1000100201015003061b0b444c00020b1a16470a4e051a4e114f1f410e08040554154f064f410c1c00180c0010000b0f5216060605165515520e09560e00064514411304094c1d0c411507001a1b45064f570b11480d001d4c134f060047541b185c";
    let s3: &str = "0b07540c1d0d0b4800354f501d131309594150010011481a1b5f11090c0145124516121d0e0c411c030c45150a16541c0a0b0d43540c411b0956124f0609075513051816590026004c061c014502410d024506150545541c450110521a111758001d0607450d11091d00121d4f0541190b45491e02171a0d49020a534f";
    let s4: &str = "031a5410000a075f5438001210110a011c5350080a0048540e431445081d521345111c041f0245174a0006040002001b01094914490f0d53014e570214021d00160d151c57420a0d03040b4550020e1e1f001d071a56110359420041000c0b06000507164506151f104514521b02000b0145411e05521c1852100a52411a0054180a1e49140c54071d5511560201491b0944111a011b14090c0e41";
    let s5: &str = "0b4916060808001a542e0002101309050a45500b00050d04005e030c071b4c1f111b161a4f01500a08490b0b451604520d0b1d1445060f531c48124f1305014c051f4c001100262d38490f0b4450061800004e001b451b1d594e45411d014e004801491b0b0602050d41041e0a4d53000d0c411c41111c184e130a0015014f03000c1148571d1c011c55034f12030d4e0b45150c5c";
    let s6: &str = "011b0d131b060d4f5233451e161b001f59411c090a0548104f431f0b48115505111d17000e02000a1e430d0d0b04115e4f190017480c14074855040a071f4448001a050110001b014c1a07024e5014094d0a1c541052110e54074541100601014e101a5c";
    let s7: &str = "0c06004316061b48002a4509065e45221654501c0a075f540c42190b165c";
    let s8: &str = "1f3cb1f3e01f3fd1f3ea1f3e61f3e01f3e71f3b31f3a91f3c81f3a91f3f91f3fc1f3fb1f3ec1f3e51f3f01f3a91f3f91f3ec1f3ec526e1b014a020411074c17111b1c071c4e4f0146430d0d08131d1d010707040017091648461e1d0618444f074c010e19594f0f1f1a07024e1d041719164e1c1652114f411645541b004e244f080213010c004c3b4c0911040e480e070b00310213101c4d0d4e00360b4f151a005253184913040e115454084f010f114554111d1a550f0d520401461f3e01f3e71f3e81f3e71f3ea1f3e01f3e81f3e51f3a91f3e01f3e71f3fa1f3fd1f3e01f3fd1f3fc1f3fd1f3e01f3e61f3e71f3a7";
    let s9: &str = "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";

    let ve = vec![
        cv(s0),
        hex::decode(s1).unwrap(),
        cv(s1),
        cv(s2),
        cv(s3),
        cv(s4),
        cv(s5),
        cv(s6),
        cv(s7),
        cv(s8),
        cv(s9),
    ];
    //let p0 = "The".as_bytes();
    //let p0 = Vec::from_hex(p0).expect("Decoding failed");
    let pa = "Governments ".as_bytes().to_vec();

    for v in &ve {
        //let pa = "The ".as_bytes().to_vec();

        let x = xor(&ve[1], v);
        let xe = xor(&pa, &x);
        let xs = std::str::from_utf8(&xe);
        println!("{:?}", xs.expect("error"));
    }

    let key = xor(&pa, &ve[1]);
    println!("key: {}", std::str::from_utf8(&key).expect("error"));

    let key = "Bitcoin: A purely peer-to-peer version of electronic cash would allow online
    payments to be sent directly from one party to another without going through a
    financial institution. Digital signatures provide part of the solution, but the main
    benefits are lost if a trusted third party is still required to prevent double-spending.
    We propose a solution to the double-spending problem using a peer-to-peer network.
    The network timestamps transactions by hashing them into an ongoing chain of
    hash-based proof-of-work, forming a record that cannot be changed without redoing
    the proof-of-work. The longest chain not only serves as proof of the sequence of
    events witnessed, but proof that it came from the largest pool of CPU power. As
    long as a majority of CPU power is controlled by nodes that are not cooperating to
    attack the network, they'll generate the longest chain and outpace attackers. The
    network itself requires minimal structure. Messages are broadcast on a best effort
    basis, and nodes can leave and rejoin the network at will, accepting the longest
    proof-of-work chain as proof of what happened while they were gone."
        .as_bytes()
        .to_vec();
}
