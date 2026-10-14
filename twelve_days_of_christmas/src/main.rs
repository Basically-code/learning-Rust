/*
    Prints the whole christmass carol "The Twelve days of christmass"
*/
fn main() {
    let mut cnt = 0;
    loop {
        if cnt == 12 {
            println!("-------END OF THE SONG---------");
            break;}
        let days: [&str;12] = ["First", "Second", "Third", "Fourth", "Fifth", "Sixth", "Seventh", "Eighth", "Ninth", "Tenth", "Eleventh", "Twelfth"];
        
        println!("\n Verse {}",cnt + 1 );

        println!("On the {} day of Christmas my true love sent to me", days[cnt]);

        let gifts: [&str;11] = [
            " Two turtle doves,",
            " Three French hens,",
            " four calling birds,",
            " five gold rings,",
            " six geese a-laying,",
            " seven swans a-swimming,",
            " eight maids a-milking,",
            " nine ladies dancing,",
            " ten lords a-leaping,",
            " eleven pipers piping,",
            " twelve drummers drumming,"
        ];

        let mut in_cnt = 0;

        loop {
            if cnt == 0 {break;}
            if in_cnt == cnt {
                break;
            }
            let gift: &str = gifts[in_cnt];
            in_cnt += 1;

            println!("{gift}");
        }

        if cnt == 0 {
            println!(" A partridge in a pear tree");
        }else {
            println!("And a partridge in a pear tree");
        }

        cnt += 1;
    }
}
