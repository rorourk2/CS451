use std::env;
use std::fs::File;
use std::io::Read;
use std::io::{self,Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len()>=3 {
        let filename = &args[1];
        let message = &args[2];
        //println!("{}", filename);
        //println!("{}", message);

        let mut file = match File::open(filename){
            Ok(line) => line,
            Err(_) => panic!("Error reading file {}",filename),
        };
        let mut data: Vec<u8> = Vec::new();
        file.read_to_end(&mut data).expect("Unable to open");


        let mut file2 = match File::open(message){
            Ok(line) => line,
            Err(_) => panic!("Error reading file {}",message),
        };
        let mut pass: Vec<u8> = Vec::new();
        file2.read_to_end(&mut pass).expect("Unable to open");
        let mut i = 3;
        let mut s=0;

        /*while s<128{
            print!("{:x?} ",data[s]);
            s+=1;
		}

        s=0;*/
        let mut z=0;
        let mut z2=7;
        while i>0 {
            if data[s]==b'\n' {
                i=i-1;     
			}
            s=s+1;
		}
        //println!("");
        //let pass: Vec<u8>= message.as_bytes().to_vec();
        while z < pass.len(){
              while z2>=0 {
                    if pass[z] & (1 << z2) != 0 {
                        if data[s] & (1 << 0) == 0{
                            data[s]=data[s] ^ 0b0000_0001;
                        }
					}else{
                        if data[s] & (1 << 0) != 0{
                            data[s]=data[s] ^ 0b0000_0001;
                        }
					}
                    z2-=1;
                    s+=1;
                    if s>=data.len(){
                        panic!("Message is too long for designated ppm to hold");
					}
			  }
              z2=7;
              z+=1;
		}
        z=0;
        while z<8{
            data[s]=data[s] & 0b1111_1110;
            s+=1;
            z+=1;
		}
        

        //let mut new_file = File::create(filename).expect("Unable to open");
       // let mut pos = 0;
        //let d:&[u8]= &data;
        /*while pos < d.len() {
            let b = newFile.write(&mut d[pos..]);
            pos += b;
        }*/
       // new_file.write(&mut data).expect("Unable to open");

        //s=0;

        //while s<data.len(){
           // print!("{:x?} ",data[s]);
            //s+=1;
		//}

        io::stdout().write(&data).expect("Unable to open");
        //io::stdout().write_fmt(format_args!("{:x?}", &data)).expect("Unable to open");
        println!("");
    }else if args.len()==2{
        let filename = &args[1];
        //println!("{}", filename);
        let mut file = match File::open(filename){
            Ok(line) => line,
            Err(_) => panic!("Error reading file {}",filename),
        };

        let mut data: Vec<u8> = Vec::new();
        
        file.read_to_end(&mut data).expect("Unable to open");
        let mut res: Vec<u8> = Vec::new();
        let mut i = 3;
        let mut s=0;
        let mut z=0;
        let mut a=128;
        let mut r=0;
        while i>0 {
            if data[s]==b'\n' {
                i=i-1;     
			}
            s=s+1;
		}

         //z+=(data[s] & (1 << 0) != 0) as u8;
        while s<data.len() {
            z+=((data[s] & (1 << 0) != 0)as u32 *a) as u8;
            
            if a==1 {
                //a=1;
                res.push(z);
                if z == b'\0' {
                    break;        
				}
                r+=1;
                z=0;
                a=128;
			}else{
                a/=2;
            }
            s+=1;
            
		}

        let mut t=0;
        while t<r{
            print!("{}", res[t] as char);
            t+=1;
		}

        println!("\n");


	}else{

        panic!("Invalid number of arguments");
    }
    
}
