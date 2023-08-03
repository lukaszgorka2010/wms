use pallet::Pallet;
mod pallet;

fn main() {
    let mut plt = pallet::add(12345554353453454345, 21372173, 220, "A2137".to_string());
    println!("{:#?}", &plt);

    plt.update_quantity(110);
    println!("{:#?}", &plt);
    
    plt.update_slot("P2137".to_string());
    println!("{:#?}", &plt);
}
