//basically the same thing as using with c++
//The main difference is that you dont have to use #include because every rust project compiles the standard library,
//and additionally dependencies are added in cargo.toml with https://crates.io
use std::collections::{LinkedList};
use std::io::Error;
use std::mem;

//notice how main returns a value type called result, there is a very good reason for this.
fn main()->Result<(),Error>
{
    //the variable file uses file io to read time_position. The ? at the end refers to the fact that the fact the
    //operation can return either the expected String, or an error in the case of a failed read. Thus is the main reason 
    //why main returns result with the two types void(represented by () ) or an Error.
    let file=std::fs::read_to_string("time_position.txt")?;
    //creating a bunch of link lists in used for calculating and iterating
    let mut x_list:LinkedList<f64>=LinkedList::new();
    let mut time_list:LinkedList<f64>=LinkedList::new();
    let mut t_list2:LinkedList<f64>=LinkedList::new();
    let mut vel_list:LinkedList<f64>=LinkedList::new();
    let mut acc_list:LinkedList<f64>=LinkedList::new();
    //iteration varaibles for keeping track of last value used
    //Variables are immutable (constant) by default in rust. To make the variable mutable, the keyword mut must be used.
    let mut x=0.;
    let mut t=0.;
        //for loop iterates through all lines in the file
    for line in file.lines(){
        //lines is an iterator for file variables
        //yeah this gets messy breaking up the string, and s convering to convert between Types.
        let mut chunks=line.splitn(2, ",");
        let a1=chunks.next().unwrap();
        let b1=chunks.next().unwrap();
        //unwrap is a function of rusts memory saftey features. most lists or iterators return Option<Type> which could or
        //could not contain data. If it does, it will convert to the called type. If not it will cause the program to panic.
        //thus unwrap is refered to a panic. expect is another panic, but allows for specific panic messages to console.
        //See program two for an example of that.
        //The ? operator often does a similar job. Works with option<T> and Result<T,Error>.
        //other versions of unwrap such as unwrap_or() can allow a default case where no value is found.
        let a2=a1.to_owned();
        let b2=b1.to_owned();
        //to_owned just converts a pointer to a owned value. to_owned and to_string are two similar functions but have specific uses
        let a3=a2.parse::<f64>().unwrap();
        let b3=b2.parse::<f64>().unwrap();
        //store from largest to smallest
        if a3>t {
        t=a3;
        x=b3;
        }
    }
    //inserts values into the list
    time_list.push_front(t); x_list.push_front(x);
    //println!("{},{}",time_list.front().unwrap(), x_list.front().unwrap()); //test line
        
    loop {
        //reusable variables for use in the serching algorithm
        let mut t_rec=0.;
        let mut x_rec=0.;
            for line in file.lines(){
                //this line adds the time position values from high values to low values on the lists stack like
                let mut chunks=line.splitn(2, ',');
                let a1=chunks.next().unwrap();
                let b1=chunks.next().unwrap();
                let a2=a1.to_owned();
                let b2=b1.to_owned();
                let a3=a2.parse::<f64>().unwrap();
                let b3=b2.parse::<f64>().unwrap();
                //i just have to do this because this is the simplies way i know how to convert a line to f64 in rust
                //thank god rust is fast
                if  a3 < *time_list.front().unwrap() && t_rec<a3
                {
                    t_rec=a3;
                    x_rec=b3;
                }
            }
            //once next highest varaibles are found set t and x equal to them
            t=t_rec;
            x=x_rec;
                
            if t<*time_list.front().unwrap() {
                
                time_list.push_front(t);
                x_list.push_front(x);
                //println!("{},{}",time_list.front().unwrap(), x_list.front().unwrap());
                continue;
            }
            else if t==*time_list.front().unwrap(){
                //for some reason the program stores 0, 0. This gets rid of those values before issues arrise.
                time_list.pop_front(); x_list.pop_front();
                break;
            }
        }
        //freeing memory when avaliable.Most likely isnt needed, but im still learning rust.
        mem::drop(file); mem::drop(t); mem::drop(x);

        //creating an iterator for t and x list
        let t_iter=time_list.iter();
        let mut x_iter=x_list.iter();
        let mut tprev=0.;
        let mut xprev=0.;
        let mut disp=0.;
        let mut dist=0.;

        for  tn in t_iter{
            let xn=x_iter.next().unwrap();
            //iterates both through x and t lists
                
            if tprev==0. && xprev==0. { // makes it so that xprev and tprev is not zero
                //println!("{},{},{},{}",*xn,xprev,*tn,tprev);
                tprev=tn.to_owned();
                xprev=xn.to_owned();
            }
            else{//calculates the displacement, distance and velocity values
                //println!("{},{},{},{}",*xn,xprev,*tn,tprev);
                vel_list.push_back((*xn-xprev)/(*tn-tprev));
                t_list2.push_back(tprev+(*tn-tprev)/2.);
                disp+=*xn-xprev;
                dist+=(*xn-xprev).abs();
                tprev=tn.to_owned();
                xprev=xn.to_owned();
                //println!("{},{}",*vel_list.back().unwrap(),t_list2.back().unwrap())
                }
        }
        mem::forget(xprev);
        //creating iterators
        let viter=vel_list.iter();
        let mut t2iter=t_list2.iter();
        tprev=0.;
        let mut vprev=0.;
        //values needed for calculations
        let mut vmax=vel_list.front().unwrap().to_owned();
        let mut vmin=vel_list.front().unwrap().to_owned();
        let mut vsum=0.;
        let mut num=0.;

        for vn in viter{
            let tn=t2iter.next().unwrap();
            vsum+=*vn;
            num+=1.;
            if *vn>vmax{
                vmax=vn.to_owned();
            }
            if *vn<vmin{
                vmin=vn.to_owned();
            }

            if tprev==0. && vprev==0. {
                //println!("{},{},{},{}",*xn,xprev,*tn,tprev);
                tprev=tn.to_owned();
                vprev=vn.to_owned();
            }
            else{
                acc_list.push_back((*vn-vprev)/(*tn-tprev));
                tprev=tn.to_owned();
                vprev=vn.to_owned();
                //println!("{},{},{},{}",*vn,vprev,*tn,tprev);
                //println!("{},{}",*vel_list.back().unwrap(),t_list2.back().unwrap());
            }
        }
        let vavg=vsum/num;
        
        let mut asum=0.;
        num=0.;
        let mut amax=acc_list.front().unwrap().to_owned();
        let mut amin=acc_list.front().unwrap().to_owned();
        for an in acc_list.iter(){
            asum+=*an;
            num+=1.;
            if *an>amax{
                amax=an.to_owned();
            }
            if *an<amin{
                amin=an.to_owned();
            }
        }
        let aavg=asum/num;
        //finally display values
        println!("Displacement is {disp}, and distance traveled is {dist}.");
        println!("Average Velocity is {vavg}, max velocity is {vmax}, and mininum velocity is {vmin}.");
        println!("Average Acceleration is {aavg}, max acceleration is {amax} and min aceleration is {amin}.");

        //Ok(()) indicates that there was no errors from file reading, and to return () because main has
        //a void return type, and there is no more chance of errors beyond this point.

        Ok(())
}