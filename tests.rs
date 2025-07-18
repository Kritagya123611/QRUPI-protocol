fn kyber_test(){
    use qrupi::kyber::Kyber;

    let kyber = Kyber::new();
    let (pk,sk)=kyber.keypair();
    let msg = b"Hello, QRUPI!";
    let (ct,ss_sender)= kyber.encrypt(&pk, msg);
    let ss_receiver = kyber.decrypt(&sk, &ct);
    assert_eq!(msg.to_vec(), ss_receiver);
}