//read from stdin
// prints out the line number of each line, as well

use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;

use std::sync::mpsc::channel;
//use std::thread;
//use std::io;

extern crate project04;
use project04::ThreadPool;

fn main() {

	
	let args: Vec<String> = env::args().collect();
    if args.len()==5 {

        let threads = args[1].parse::<usize>().unwrap();
        let filename = &args[2];
        let input = &args[3];
        let output2 = &args[4];
        //println!("{}", filename);

        let mut file = match File::open(filename){
            Ok(line) => line,
            Err(_) => panic!("Error reading file {}",filename),
        };
        let mut word: Vec<u8> = Vec::new();
        file.read_to_end(&mut word).expect("Unable to open"); //important

        println!("##########");

        /*let check_file = match File::open("logo.ppm"){
            Ok(line) => line,
            Err(_) => panic!("Error reading file logo.ppm"),
        };*/

        /*let mut i = 3;
        let mut s=0;
        let mut z=0;
        let mut z2=7;
        while i>0 {
            if data[s]==b'\n' {
                i=i-1;     
			}
            s=s+1;
		}
        let mut s2=s;
        i=0;
        while data[s2]!=b'\0'{
            i=i+1;
            s2=s2+1;
		}*/
        //let mut input2 = input.to_string();
        println!("# message.len() = {}", word.len());//important
        println!("##########");
        println!("**********");
        println!("* Checking input directory for valid PPMs");

        //let paths = std::fs::read_dir("/home/user");
        let mut ppm: Vec<String> = Vec::new();
        let mut p_num=0;
        
        for photo in std::fs::read_dir(input).unwrap() {
            let &dir = &photo.as_ref();
            //file_type(&self)
            //if dir.unwrap().file_name().contains(".ppm") {
           //if &dir.unwrap().file_type().unwrap() == &check_file.metadata().unwrap().file_type(){
           if dir.unwrap().file_name().to_string_lossy().contains(".ppm"){
                /*let v = match libsteg::PPM::new(dir.unwrap().file_name().to_string_lossy().to_string()) {
                    Ok(v) => v,
                    Err(err) => panic!("Error: {:?}", err),
                };*/
                ppm.push(input.to_string()+"/"+&dir.unwrap().file_name().to_string_lossy().to_string());

                println!("Found valid PPM: {}", photo.unwrap().path().display());
                p_num=p_num+1;

            }else{
                println!("Potential input file not a valid PPM: {}", photo.unwrap().path().display());
			}

        }
        
        let mut div: Vec<usize> = Vec::new();
        if threads <= word.len(){
            div.push(0);
            let division = word.len() /threads;
            let mut x=1;
            while x < threads {
                div.push(division*x);
                x=x+1;        
			}
            div.push(word.len());
        
        } else {
              panic!("Too many Threads for message length");
		}
        

        let thread_pool = ThreadPool::new(threads);

        //let mut num =1;
       //let mut children = vec![];
        let mut end_file:String = "".to_string();
        let mut wer=1;
        //let mut &ppmc: Vec<String>= &ppm;
        for y in 0..threads {
            let ppmc: Vec<String>= ppm.clone();
            let pass: Vec<u8>= word.clone();
            let star=div[y];
            let end=div[y+1];
            let into_file=end_file.as_str().to_string();
            let output = output2.to_string();
            
            //children.push(thread::spawn(move || {
            thread_pool.execute(move || {   
                let mut pic_num=0;
                println!("Spawning Thread #{}", y+1);
                let mut fin=false;
                let mut z=star;
                let mut flin="00000000";


                let mut round=1;
                //let input3=input2;
                while fin==false {
                    //input2=input3;
                    let mut file2 = match File::open(&ppmc[pic_num]){
                        Ok(line) => line,
                        Err(_) => panic!("Error reading file {}",&ppmc[pic_num]),
                    };
                    let mut data: Vec<u8> = Vec::new();
                    file2.read_to_end(&mut data).expect("Unable to open");
                    let mut s=0;
                    let mut i = 3;
                    let mut z2=7;
                    while i>0 {
                        if data[s]==b'\n' {
                            i=i-1;     
			            }
                        s=s+1;
		            }
                



                    while z < end{
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
                            
			            }
                        z2=7;
                        z+=1;
                        if s+15>=data.len(){
                                break;
					    }
		            }
                    let mut z3=0;
                    while z3<8{
                        data[s]=data[s] & 0b1111_1110;
                        s+=1;
                        z3+=1;
		            }
                    if pic_num+1 == ppmc.len() {
                        pic_num = 0;           
					}else{
                        pic_num = pic_num+1;           
					}


                    if z>=end {
                        fin = true;           
					}

                    //999999999
                    
                    let mut out_file = std::fs::File::create(output.to_string()+"/"+&into_file.to_string()+&flin.to_string()+&round.to_string()+".ppm").expect("create failed");

                    out_file.write(&data);
                    
                    /*if round%10==9 {
                        round=round*10  
		            }*/

                    round=round+1; 

                    if round==10{
                        flin="0000000";          
					}
                    if round==100{
                        flin="000000";          
					}
                    if round==1000{
                        flin="00000";           
					}
                    if round==10000{
                        flin="0000";           
					}
                    if round==100000{
                        flin="000";           
					}
                    if round==1000000{
                        flin="00";           
					}
                    if round==10000000{
                        flin="0";           
					}
                    if round==100000000{
                        flin="";           
					}
                }





                /*for x in ppmc{
                    println!("{}", x);
				}*/
            });
           
           



           if wer%26==1 {
                end_file=end_file+"a";
           }else if wer%26==2 {
                        end_file.pop();
                        end_file=end_file+"b";
                    }else if wer%26==3 {
                        end_file.pop();
                        end_file=end_file+"c";
                    }else if wer%26==4 {
                        end_file.pop();
                        end_file=end_file+"d";
                    }else if wer%26==5 {
                        end_file.pop();
                        end_file=end_file+"e";
                    }else if wer%26==6 {
                        end_file.pop();
                        end_file=end_file+"f";
                    }else if wer%26==7 {
                        end_file.pop();
                        end_file=end_file+"g";
                    }else if wer%26==8 {
                        end_file.pop();
                        end_file=end_file+"h";
                    }else if wer%26==9 {
                        end_file.pop();
                        end_file=end_file+"i";
                    }else if wer%26==10 {
                        end_file.pop();
                        end_file=end_file+"j";
                    }else if wer%26==11 {
                        end_file.pop();
                        end_file=end_file+"k";
                    }else if wer%26==12 {
                        end_file.pop();
                        end_file=end_file+"l";
                    }else if wer%26==13 {
                        end_file.pop();
                        end_file=end_file+"m";
                    }else if wer%26==14 {
                        end_file.pop();
                        end_file=end_file+"n";
                    }else if wer%26==15 {
                        end_file.pop();
                        end_file=end_file+"o";
                    }else if wer%26==16 {
                        end_file.pop();
                        end_file=end_file+"p";
                    }else if wer%26==17 {
                        end_file.pop();
                        end_file=end_file+"q";
                    }else if wer%26==18 {
                        end_file.pop();
                        end_file=end_file+"r";
                    }else if wer%26==19 {
                        end_file.pop();
                        end_file=end_file+"s";
                    }else if wer%26==20 {
                        end_file.pop();
                        end_file=end_file+"t";
                    }else if wer%26==21 {
                        end_file.pop();
                        end_file=end_file+"u";
                    }else if wer%26==22 {
                        end_file.pop();
                        end_file=end_file+"v";
                    }else if wer%26==23 {
                        end_file.pop();
                        end_file=end_file+"w";
                    }else if wer%26==24 {
                        end_file.pop();
                        end_file=end_file+"x";
                    }else if wer%26==25 {
                        end_file.pop();
                        end_file=end_file+"y";
                    }else if wer%26==26 {
                        end_file.pop();
                        end_file=end_file+"z";
                    }


            wer=wer+1;





            /*thread::spawn(|| {
                println!("this is thread number {}", t);
                for x in ppmc{
                            println!("{}", x);
				}
            });*/

        }

        /*for child in children {
            let _ = child.join();
        }*/





    }else if args.len()==3 {
        let threads = args[1].parse::<usize>().unwrap();
        let input = &args[2];

        //let mut input2 = input.to_string();

        let mut ppm: Vec<String> = Vec::new();
        let mut p_num=0;
        
        for photo in std::fs::read_dir(input).unwrap() {
            let &dir = &photo.as_ref();
            //file_type(&self)
            //if dir.unwrap().file_name().contains(".ppm") {
           //if &dir.unwrap().file_type().unwrap() == &check_file.metadata().unwrap().file_type(){
           if dir.unwrap().file_name().to_string_lossy().contains(".ppm"){
                /*let v = match libsteg::PPM::new(dir.unwrap().file_name().to_string_lossy().to_string()) {
                    Ok(v) => v,
                    Err(err) => panic!("Error: {:?}", err),
                };*/
                ppm.push(input.to_string()+"/"+&dir.unwrap().file_name().to_string_lossy().to_string());

                //println!("Found valid PPM: {}", photo.unwrap().path().display());
                p_num=p_num+1;

            }

        }



        //let mut division;
        let mut div: Vec<usize> = Vec::new();
        if threads <= ppm.len(){
            div.push(0);
            let division = ppm.len() /threads;
            let mut x=1;
            while x < threads {
                div.push(division*x);
                x=x+1;        
			}
            div.push(ppm.len());
        
        } else {
              panic!("Too many Threads for message length");
		}

        let thread_pool = ThreadPool::new(threads);
        //let mut children = vec![];
        //let mut sen: Vec<std::sync::mpsc::Sender<String>> = Vec::new();
        let mut rec: Vec<std::sync::mpsc::Receiver<Vec<u8>>> = Vec::new();
        for y in 0..threads {
            let ppmc: Vec<String>= ppm.clone();
            let star=div[y];
            let end=div[y+1];
            let (tx, rx): (std::sync::mpsc::Sender<Vec<u8>>, std::sync::mpsc::Receiver<Vec<u8>>) = channel();
            rec.push(rx);
            //children.push(thread::spawn(move || {
            thread_pool.execute(move || {
                let mut st=star;
                //let mut deco:String = "".to_string();

                let mut res: Vec<u8> = Vec::new();//


                    while st <end{
                        let mut file = match File::open(&ppmc[st]){
                            Ok(line) => line,
                            Err(_) => panic!("Error reading file {}",&ppmc[st]),
                        };

                        let mut data: Vec<u8> = Vec::new();
        
                        file.read_to_end(&mut data).expect("Unable to open");
                        //let mut res: Vec<u8> = Vec::new();
                        let mut i = 3;
                        let mut s=0;
                        let mut z=0;
                        let mut a=128;
                        //let mut r=0;
                        while i>0 {
                            if data[s]==b'\n' {
                                i=i-1;     
			                }
                            s=s+1;
		                }
                        while s<data.len() {
                            z+=((data[s] & (1 << 0) != 0)as u32 *a) as u8;
            
                            if a==1 {
                                res.push(z);
                                if z == b'\0' {
                                    break;        
				                }
                                //r+=1;
                                z=0;
                                a=128;
			                }else{
                                a/=2;
                            }
                            s+=1;
            
		                }

                        //deco = deco + &String::from_utf8_lossy(&res);
                        
                        st=st+1;
					}

                    /*let mut gett=0;
                    while gett <res.len(){
                        print!("{}", res[gett] as char);
                        gett=gett+1;
				    }*/
                    tx.send(res).unwrap();
            });
        }

        let mut gtr=0;
        while gtr < rec.len(){
            let apricots: Vec<u8>=rec[gtr].recv().unwrap();
            let mut gett=0;
                while gett <apricots.len(){
                    print!("{}", apricots[gett] as char);
                    gett=gett+1;
				}
            gtr=gtr+1;
		}
        //println!("size {}", rec.len());

    }else{
        panic!("Invalid number of arguments");

	}
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	/*let thread_pool = ThreadPool::new(5);

	//get stdin
	let reader = io::stdin();

	//keep track of number of lines read
	let mut lines_read = 0;

	//we need a variable to store the line that is being read in
	let mut line = String::new();

	//returns number of bytes read
	//reader.read_line(&mut String)

	while reader.read_line(&mut line).unwrap() > 0{
		lines_read += 1;

		let cloned_line = line.clone();

		thread_pool.execute(move || {
			process_line(lines_read, &cloned_line);
		});

		line.clear();
	}*/
}

/*fn process_line(line_number: u32, line: &str){
	print!("line number {}: {}", line_number, line);
}*/