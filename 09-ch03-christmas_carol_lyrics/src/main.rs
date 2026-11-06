fn main() {
    print_lyrics();
}

fn print_lyrics() {

    for day in 1..=6 {

        if day == 1 {
            println!("
On the first day of Christmas
My true love gave to me
A partridge in a pear tree.
");
            continue;
        }
        
        if day == 2 {
            println!("
On the second day of Christmas
My true love gave to me
Two turtle doves
And a partridge in a pear tree.
");
            continue;
        }

        if day == 3 {
            println!("
On the third day of Christmas
My true love gave to me
Three French hens,
Two turtle doves
And a partridge in a pear tree.
");
            continue;
        }

        if day == 4 {
            println!("
On the fourth day of Christmas
My true love gave to me
Four calling birds,
Three French hens,
Two turtle doves
And a partridge in a pear tree.
");
            continue;
        }

        if day == 5 {
            println!("
On the fifth day of Christmas
My true love gave to me
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves
And a partridge in a pear tree.
");
            continue;
        }

        if day == 6 {
            println!("
On the sixth day of Christmas
My true love gave to me
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves
And a partridge in a pear tree.
");
            continue;
        }
    }
}