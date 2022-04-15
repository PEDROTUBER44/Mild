use std::{
    process::{
        exit // Importing the standard exit library to exit the program
    },
    env, // Importing the standard env library to capture user arguments
    io // Importing the standard io (Input & Output) library to capture user input
};
mod texts;
mod utils;
//mod lib;

fn main() {

    let args: Vec<String> = env::args().collect();
    let option = &args[1].trim();

    let all_packages_to_remove_archlinux: &str = "";
    let all_packages_to_install_archlinux_lxde: &str = "";
    let all_packages_to_install_archlinux_lxqt: &str = "";
    let all_packages_to_install_archlinux_xfce: &str = "";
    let all_packages_to_install_archlinux_gnome: &str = "";
    let all_packages_to_install_archlinux_cinnamon: &str = "";
    let all_packages_to_install_archlinux_mate: &str = "";
    let all_packages_to_install_archlinux_kdeplasma: &str = "";
    let all_packages_to_install_archlinux_bspwm: &str = "";
    let all_packages_to_install_archlinux_cutefish: &str = "";

    let all_packages_to_remove_debian: &str = "";
    let all_packages_to_install_debian_lxde: &str = "";
    let all_packages_to_install_debian_lxqt: &str = "";
    let all_packages_to_install_debian_xfce: &str = "";
    let all_packages_to_install_debian_gnome: &str = "";
    let all_packages_to_install_debian_cinnamon: &str = "";
    let all_packages_to_install_debian_mate: &str = "";
    let all_packages_to_install_debian_kdeplasma: &str = "";
    let all_packages_to_install_debian_bspwm: &str = "";
    let all_packages_to_install_debian_cutefish: &str = "";

    let all_packages_to_remove_fedora: &str = "";
    let all_packages_to_install_fedora_lxde: &str = "";
    let all_packages_to_install_fedora_lxqt: &str = "";
    let all_packages_to_install_fedora_xfce: &str = "";
    let all_packages_to_install_fedora_gnome: &str = "";
    let all_packages_to_install_fedora_cinnamon: &str = "";
    let all_packages_to_install_fedora_mate: &str = "";
    let all_packages_to_install_fedora_kdeplasma: &str = "";
    let all_packages_to_install_fedora_bspwm: &str = "";
    let all_packages_to_install_fedora_cutefish: &str = "";

    match &option[..] {

        "--install-arch-lxde" => {

            utils::show_the_changes_that_will_be_made_to_user(all_packages_to_remove_archlinux, all_packages_to_install_archlinux_lxde);
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => utils::exec_installation("archlinux","lxde"),
                "Y" => utils::exec_installation("archlinux","lxde"),
                "" => utils::exec_installation("archlinux","lxde"),
                "n" => {println!("Aborted installation");exit(0);},
                "N" => {println!("Aborted installation");exit(0);},
                _ => {println!("Aborted installation");exit(0);}
            }

        },

        "--install-arch-lxqt" => {

            utils::show_the_changes_that_will_be_made_to_user(all_packages_to_remove_archlinux, all_packages_to_install_archlinux_lxqt);
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => utils::exec_installation("archlinux","lxqt"),
                "Y" => utils::exec_installation("archlinux","lxqt"),
                "" => utils::exec_installation("archlinux","lxqt"),
                "n" => {println!("Aborted installation");exit(0);},
                "N" => {println!("Aborted installation");exit(0);},
                _ => {println!("Aborted installation");exit(0);}
            }

        },

        "--install-arch-xfce" => {

            utils::show_the_changes_that_will_be_made_to_user(all_packages_to_remove_archlinux, all_packages_to_install_archlinux_xfce);
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => utils::exec_installation("archlinux","xfce"),
                "Y" => utils::exec_installation("archlinux","xfce"),
                "" => utils::exec_installation("archlinux","xfce"),
                "n" => {println!("Aborted installation");exit(0);},
                "N" => {println!("Aborted installation");exit(0);},
                _ => {println!("Aborted installation");exit(0);}
            }

        },

        "--install-arch-gnome" => {

            utils::show_the_changes_that_will_be_made_to_user(all_packages_to_remove_archlinux, all_packages_to_install_archlinux_gnome);
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => utils::exec_installation("archlinux","gnome"),
                "Y" => utils::exec_installation("archlinux","gnome"),
                "" => utils::exec_installation("archlinux","gnome"),
                "n" => {println!("Aborted installation");exit(0);},
                "N" => {println!("Aborted installation");exit(0);},
                _ => {println!("Aborted installation");exit(0);}
            }

        },

        "--install-arch-cinnamon" => {

            utils::show_the_changes_that_will_be_made_to_user(all_packages_to_remove_archlinux, all_packages_to_install_archlinux_cinnamon);
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => utils::exec_installation("archlinux","cinnamon"),
                "Y" => utils::exec_installation("archlinux","cinnamon"),
                "" => utils::exec_installation("archlinux","cinnamon"),
                "n" => {println!("Aborted installation");exit(0);},
                "N" => {println!("Aborted installation");exit(0);},
                _ => {println!("Aborted installation");exit(0);}
            }

        },

        "--install-arch-mate" => {

            utils::show_the_changes_that_will_be_made_to_user(all_packages_to_remove_archlinux, all_packages_to_install_archlinux_mate);
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => utils::exec_installation("archlinux","mate"),
                "Y" => utils::exec_installation("archlinux","mate"),
                "" => utils::exec_installation("archlinux","mate"),
                "n" => {println!("Aborted installation");exit(0);},
                "N" => {println!("Aborted installation");exit(0);},
                _ => {println!("Aborted installation");exit(0);}
            }

        },

        "--install-arch-kdeplasma" => {

            utils::show_the_changes_that_will_be_made_to_user(all_packages_to_remove_archlinux, all_packages_to_install_archlinux_kdeplasma);
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => utils::exec_installation("archlinux","kdeplasma"),
                "Y" => utils::exec_installation("archlinux","kdeplasma"),
                "" => utils::exec_installation("archlinux","kdeplasma"),
                "n" => {println!("Aborted installation");exit(0);},
                "N" => {println!("Aborted installation");exit(0);},
                _ => {println!("Aborted installation");exit(0);}
            }

        },

        "--install-arch-bspwm" => {

            utils::show_the_changes_that_will_be_made_to_user(all_packages_to_remove_archlinux, all_packages_to_install_archlinux_bspwm);
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => utils::exec_installation("archlinux","bspwm"),
                "Y" => utils::exec_installation("archlinux","bspwm"),
                "" => utils::exec_installation("archlinux","bspwm"),
                "n" => {println!("Aborted installation");exit(0);},
                "N" => {println!("Aborted installation");exit(0);},
                _ => {println!("Aborted installation");exit(0);}
            }

        },

        "--install-arch-cutefish" => {

            utils::show_the_changes_that_will_be_made_to_user(all_packages_to_remove_archlinux, all_packages_to_install_archlinux_cutefish);
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => utils::exec_installation("archlinux","cutefish"),
                "Y" => utils::exec_installation("archlinux","cutefish"),
                "" => utils::exec_installation("archlinux","cutefish"),
                "n" => {println!("Aborted installation");exit(0);},
                "N" => {println!("Aborted installation");exit(0);},
                _ => {println!("Aborted installation");exit(0);}
            }

        },

        "--clean-arch" => utils::clean_system("archlinux"),


        "--install-debian-lxde" => {

            utils::show_the_changes_that_will_be_made_to_user(all_packages_to_remove_debian, all_packages_to_install_debian_lxde);
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => utils::exec_installation("debian","gnome"),
                "Y" => utils::exec_installation("debian","gnome"),
                "" => utils::exec_installation("debian","gnome"),
                "n" => {println!("Aborted installation");exit(0);},
                "N" => {println!("Aborted installation");exit(0);},
                _ => {println!("Aborted installation");exit(0);}
            }

        },

        "--install-debian-lxqt" => {

            utils::show_the_changes_that_will_be_made_to_user(all_packages_to_remove_debian, all_packages_to_install_debian_lxqt);
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => utils::exec_installation("debian","lxqt"),
                "Y" => utils::exec_installation("debian","lxqt"),
                "" => utils::exec_installation("debian","lxqt"),
                "n" => {println!("Aborted installation");exit(0);},
                "N" => {println!("Aborted installation");exit(0);},
                _ => {println!("Aborted installation");exit(0);}
            }

        },

        "--install-debian-xfce" => {

            utils::show_the_changes_that_will_be_made_to_user(all_packages_to_remove_debian, all_packages_to_install_debian_xfce);
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => utils::exec_installation("debian","xfce"),
                "Y" => utils::exec_installation("debian","xfce"),
                "" => utils::exec_installation("debian","xfce"),
                "n" => {println!("Aborted installation");exit(0);},
                "N" => {println!("Aborted installation");exit(0);},
                _ => {println!("Aborted installation");exit(0);}
            }

        },

        "--install-debian-gnome" => {

            utils::show_the_changes_that_will_be_made_to_user(all_packages_to_remove_debian, all_packages_to_install_debian_gnome);
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => utils::exec_installation("debian","gnome"),
                "Y" => utils::exec_installation("debian","gnome"),
                "" => utils::exec_installation("debian","gnome"),
                "n" => {println!("Aborted installation");exit(0);},
                "N" => {println!("Aborted installation");exit(0);},
                _ => {println!("Aborted installation");exit(0);}
            }

        },

        "--install-debian-cinnamon" => {

            utils::show_the_changes_that_will_be_made_to_user(all_packages_to_remove_debian, all_packages_to_install_debian_cinnamon);
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => utils::exec_installation("debian","cinnamon"),
                "Y" => utils::exec_installation("debian","cinnamon"),
                "" => utils::exec_installation("debian","cinnamon"),
                "n" => {println!("Aborted installation");exit(0);},
                "N" => {println!("Aborted installation");exit(0);},
                _ => {println!("Aborted installation");exit(0);}
            }

        },

        "--install-debian-mate" => {

            utils::show_the_changes_that_will_be_made_to_user(all_packages_to_remove_debian, all_packages_to_install_debian_mate);
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => utils::exec_installation("debian","mate"),
                "Y" => utils::exec_installation("debian","mate"),
                "" => utils::exec_installation("debian","mate"),
                "n" => {println!("Aborted installation");exit(0);},
                "N" => {println!("Aborted installation");exit(0);},
                _ => {println!("Aborted installation");exit(0);}
            }

        },

        "--install-debian-kdeplasma" => {

            utils::show_the_changes_that_will_be_made_to_user(all_packages_to_remove_debian, all_packages_to_install_debian_kdeplasma);
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => utils::exec_installation("debian","kdeplasma"),
                "Y" => utils::exec_installation("debian","kdeplasma"),
                "" => utils::exec_installation("debian","kdeplasma"),
                "n" => {println!("Aborted installation");exit(0);},
                "N" => {println!("Aborted installation");exit(0);},
                _ => {println!("Aborted installation");exit(0);}
            }

        },

        "--install-debian-bspwm" => {

            utils::show_the_changes_that_will_be_made_to_user(all_packages_to_remove_debian, all_packages_to_install_debian_bspwm);
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => utils::exec_installation("debian","bspwm"),
                "Y" => utils::exec_installation("debian","bspwm"),
                "" => utils::exec_installation("debian","bspwm"),
                "n" => {println!("Aborted installation");exit(0);},
                "N" => {println!("Aborted installation");exit(0);},
                _ => {println!("Aborted installation");exit(0);}
            }

        },

        "--install-debian-cutefish" => {

            utils::show_the_changes_that_will_be_made_to_user(all_packages_to_remove_debian, all_packages_to_install_debian_cutefish);
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => utils::exec_installation("debian","cutefish"),
                "Y" => utils::exec_installation("debian","cutefish"),
                "" => utils::exec_installation("debian","cutefish"),
                "n" => {println!("Aborted installation");exit(0);},
                "N" => {println!("Aborted installation");exit(0);},
                _ => {println!("Aborted installation");exit(0);}
            }

        },

        "--clean-debian" => utils::clean_system("debian"),


        "--install-fedora-lxde" => {

            utils::show_the_changes_that_will_be_made_to_user(all_packages_to_remove_fedora, all_packages_to_install_fedora_lxde);
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => utils::exec_installation("fedora","lxde"),
                "Y" => utils::exec_installation("fedora","lxde"),
                "" => utils::exec_installation("fedora","lxde"),
                "n" => {println!("Aborted installation");exit(0);},
                "N" => {println!("Aborted installation");exit(0);},
                _ => {println!("Aborted installation");exit(0);}
            }

        },

        "--install-fedora-lxqt" => {

            utils::show_the_changes_that_will_be_made_to_user(all_packages_to_remove_fedora, all_packages_to_install_fedora_lxqt);
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => utils::exec_installation("fedora","lxqt"),
                "Y" => utils::exec_installation("fedora","lxqt"),
                "" => utils::exec_installation("fedora","lxqt"),
                "n" => {println!("Aborted installation");exit(0);},
                "N" => {println!("Aborted installation");exit(0);},
                _ => {println!("Aborted installation");exit(0);}
            }

        },

        "--install-fedora-xfce" => {

            utils::show_the_changes_that_will_be_made_to_user(all_packages_to_remove_fedora, all_packages_to_install_fedora_xfce);
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => utils::exec_installation("fedora","lxqt"),
                "Y" => utils::exec_installation("fedora","lxqt"),
                "" => utils::exec_installation("fedora","lxqt"),
                "n" => {println!("Aborted installation");exit(0);},
                "N" => {println!("Aborted installation");exit(0);},
                _ => {println!("Aborted installation");exit(0);}
            }

        },

        "--install-fedora-gnome" => {

            utils::show_the_changes_that_will_be_made_to_user(all_packages_to_remove_fedora, all_packages_to_install_fedora_gnome);
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => utils::exec_installation("fedora","gnome"),
                "Y" => utils::exec_installation("fedora","gnome"),
                "" => utils::exec_installation("fedora","gnome"),
                "n" => {println!("Aborted installation");exit(0);},
                "N" => {println!("Aborted installation");exit(0);},
                _ => {println!("Aborted installation");exit(0);}
            }

        },

        "--install-fedora-cinnamon" => {

            utils::show_the_changes_that_will_be_made_to_user(all_packages_to_remove_fedora, all_packages_to_install_fedora_cinnamon);
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => utils::exec_installation("fedora","cinnamon"),
                "Y" => utils::exec_installation("fedora","cinnamon"),
                "" => utils::exec_installation("fedora","cinnamon"),
                "n" => {println!("Aborted installation");exit(0);},
                "N" => {println!("Aborted installation");exit(0);},
                _ => {println!("Aborted installation");exit(0);}
            }

        },

        "--install-fedora-mate" => {

            utils::show_the_changes_that_will_be_made_to_user(all_packages_to_remove_fedora, all_packages_to_install_fedora_mate);
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => utils::exec_installation("fedora","mate"),
                "Y" => utils::exec_installation("fedora","mate"),
                "" => utils::exec_installation("fedora","mate"),
                "n" => {println!("Aborted installation");exit(0);},
                "N" => {println!("Aborted installation");exit(0);},
                _ => {println!("Aborted installation");exit(0);}
            }

        },

        "--install-fedora-kdeplasma" => {

            utils::show_the_changes_that_will_be_made_to_user(all_packages_to_remove_fedora, all_packages_to_install_fedora_kdeplasma);
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => utils::exec_installation("fedora","kdeplasma"),
                "Y" => utils::exec_installation("fedora","kdeplasma"),
                "" => utils::exec_installation("fedora","kdeplasma"),
                "n" => {println!("Aborted installation");exit(0);},
                "N" => {println!("Aborted installation");exit(0);},
                _ => {println!("Aborted installation");exit(0);}
            }

        },

        "--install-fedora-bspwm" => {

            utils::show_the_changes_that_will_be_made_to_user(all_packages_to_remove_fedora, all_packages_to_install_fedora_bspwm);
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => utils::exec_installation("fedora","bspwm"),
                "Y" => utils::exec_installation("fedora","bspwm"),
                "" => utils::exec_installation("fedora","bspwm"),
                "n" => {println!("Aborted installation");exit(0);},
                "N" => {println!("Aborted installation");exit(0);},
                _ => {println!("Aborted installation");exit(0);}
            }

        },

        "--install-fedora-cutefish" => {

            utils::show_the_changes_that_will_be_made_to_user(all_packages_to_remove_fedora, all_packages_to_install_fedora_cutefish);
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => utils::exec_installation("fedora","cutefish"),
                "Y" => utils::exec_installation("fedora","cutefish"),
                "" => utils::exec_installation("fedora","cutefish"),
                "n" => {println!("Aborted installation");exit(0);},
                "N" => {println!("Aborted installation");exit(0);},
                _ => {println!("Aborted installation");exit(0);}
            }

        },

        "--clean-fedora" => utils::clean_system("fedora"),


        "--help" => {

            println!("{}",texts::HELP_EN_US);

        },

        _ => {

            println!("{}",texts::HELP_EN_US);

        }

    }

}