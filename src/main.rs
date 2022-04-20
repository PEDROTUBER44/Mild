use std::{
    process::{
        exit // Importing the standard exit library to exit the program
    },
    env // Importing the standard env library to capture user arguments
};
mod texts;
mod utils;
//mod lib;

fn main() {

    let args: Vec<String> = env::args().collect();
    let option = &args[1].trim();

    match &option[..] {

        "--install-arch-lxde" => {

            utils::show_the_changes_that_will_be_made_to_user(texts::ALL_PACKAGES_TO_REMOVE_ARCHLINUX, texts::ALL_PACKAGES_TO_INSTALL_ARCHLINUX_LXDE);
            utils::exec_installation("archlinux","lxde");
            exit(0);

        },

        "--install-arch-lxqt" => {

            utils::show_the_changes_that_will_be_made_to_user(texts::ALL_PACKAGES_TO_REMOVE_ARCHLINUX, texts::ALL_PACKAGES_TO_INSTALL_ARCHLINUX_LXQT);
            utils::exec_installation("archlinux","lxqt");
            exit(0);

        },

        "--install-arch-xfce" => {

            utils::show_the_changes_that_will_be_made_to_user(texts::ALL_PACKAGES_TO_REMOVE_ARCHLINUX, texts::ALL_PACKAGES_TO_INSTALL_ARCHLINUX_XFCE);
            utils::exec_installation("archlinux","xfce");
            exit(0);

        },

        "--install-arch-gnome" => {

            utils::show_the_changes_that_will_be_made_to_user(texts::ALL_PACKAGES_TO_REMOVE_ARCHLINUX, texts::ALL_PACKAGES_TO_INSTALL_ARCHLINUX_GNOME);
            utils::exec_installation("archlinux","gnome");
            exit(0);

        },

        "--install-arch-cinnamon" => {

            utils::show_the_changes_that_will_be_made_to_user(texts::ALL_PACKAGES_TO_REMOVE_ARCHLINUX, texts::ALL_PACKAGES_TO_INSTALL_ARCHLINUX_CINNAMON);
            utils::exec_installation("archlinux","cinnamon");
            exit(0);

        },

        "--install-arch-mate" => {

            utils::show_the_changes_that_will_be_made_to_user(texts::ALL_PACKAGES_TO_REMOVE_ARCHLINUX, texts::ALL_PACKAGES_TO_INSTALL_ARCHLINUX_MATE);
            utils::exec_installation("archlinux","mate");
            exit(0);

        },

        "--install-arch-kdeplasma" => {

            utils::show_the_changes_that_will_be_made_to_user(texts::ALL_PACKAGES_TO_REMOVE_ARCHLINUX, texts::ALL_PACKAGES_TO_INSTALL_ARCHLINUX_KDEPLASMA);
            utils::exec_installation("archlinux","kdeplasma");
            exit(0);

        },

        "--install-arch-bspwm" => {

            utils::show_the_changes_that_will_be_made_to_user(texts::ALL_PACKAGES_TO_REMOVE_ARCHLINUX, texts::ALL_PACKAGES_TO_INSTALL_ARCHLINUX_BSPWM);
            utils::exec_installation("archlinux","bspwm");
            exit(0);

        },

        "--install-arch-cutefish" => {

            utils::show_the_changes_that_will_be_made_to_user(texts::ALL_PACKAGES_TO_REMOVE_ARCHLINUX, texts::ALL_PACKAGES_TO_INSTALL_ARCHLINUX_CUTEFISH);
            utils::exec_installation("archlinux","cutefish");
            exit(0);

        },

        "--clean-arch" => utils::clean_system("archlinux"),


        "--install-debian-lxde" => {

            utils::show_the_changes_that_will_be_made_to_user(texts::ALL_PACKAGES_TO_REMOVE_DEBIAN, texts::ALL_PACKAGES_TO_INSTALL_DEBIAN_LXDE);
            utils::exec_installation("debian","lxde");
            exit(0);

        },

        "--install-debian-lxqt" => {

            utils::show_the_changes_that_will_be_made_to_user(texts::ALL_PACKAGES_TO_REMOVE_DEBIAN, texts::ALL_PACKAGES_TO_INSTALL_DEBIAN_LXQT);
            utils::exec_installation("debian","lxqt");
            exit(0);

        },

        "--install-debian-xfce" => {

            utils::show_the_changes_that_will_be_made_to_user(texts::ALL_PACKAGES_TO_REMOVE_DEBIAN, texts::ALL_PACKAGES_TO_INSTALL_DEBIAN_XFCE);
            utils::exec_installation("debian","xfce");
            exit(0);

        },

        "--install-debian-gnome" => {

            utils::show_the_changes_that_will_be_made_to_user(texts::ALL_PACKAGES_TO_REMOVE_DEBIAN, texts::ALL_PACKAGES_TO_INSTALL_DEBIAN_GNOME);
            utils::exec_installation("debian","gnome");
            exit(0);

        },

        "--install-debian-cinnamon" => {

            utils::show_the_changes_that_will_be_made_to_user(texts::ALL_PACKAGES_TO_REMOVE_DEBIAN, texts::ALL_PACKAGES_TO_INSTALL_DEBIAN_CINNAMON);
            utils::exec_installation("debian","cinnamon");
            exit(0);

        },

        "--install-debian-mate" => {

            utils::show_the_changes_that_will_be_made_to_user(texts::ALL_PACKAGES_TO_REMOVE_DEBIAN, texts::ALL_PACKAGES_TO_INSTALL_DEBIAN_MATE);
            utils::exec_installation("debian","mate");
            exit(0);

        },

        "--install-debian-kdeplasma" => {

            utils::show_the_changes_that_will_be_made_to_user(texts::ALL_PACKAGES_TO_REMOVE_DEBIAN, texts::ALL_PACKAGES_TO_INSTALL_DEBIAN_KDEPLASMA);
            utils::exec_installation("debian","kdeplasma");
            exit(0);

        },

        "--install-debian-bspwm" => {

            utils::show_the_changes_that_will_be_made_to_user(texts::ALL_PACKAGES_TO_REMOVE_DEBIAN, texts::ALL_PACKAGES_TO_INSTALL_DEBIAN_BSPWM);
            utils::exec_installation("debian","bspwm");
            exit(0);

        },

        "--install-debian-cutefish" => {

            utils::show_the_changes_that_will_be_made_to_user(texts::ALL_PACKAGES_TO_REMOVE_DEBIAN, texts::ALL_PACKAGES_TO_INSTALL_DEBIAN_CUTEFISH);
            utils::exec_installation("debian","cutefish");
            exit(0);

        },

        "--clean-debian" => utils::clean_system("debian"),


        "--install-fedora-lxde" => {

            utils::show_the_changes_that_will_be_made_to_user(texts::ALL_PACKAGES_TO_REMOVE_FEDORA, texts::ALL_PACKAGES_TO_INSTALL_FEDORA_LXDE);
            utils::exec_installation("fedora","lxde");
            exit(0);

        },

        "--install-fedora-lxqt" => {

            utils::show_the_changes_that_will_be_made_to_user(texts::ALL_PACKAGES_TO_REMOVE_FEDORA, texts::ALL_PACKAGES_TO_INSTALL_FEDORA_LXQT);
            utils::exec_installation("fedora","lxqt");
            exit(0);

        },

        "--install-fedora-xfce" => {

            utils::show_the_changes_that_will_be_made_to_user(texts::ALL_PACKAGES_TO_REMOVE_FEDORA, texts::ALL_PACKAGES_TO_INSTALL_FEDORA_XFCE);
            utils::exec_installation("fedora","xfce");
            exit(0);

        },

        "--install-fedora-gnome" => {

            utils::show_the_changes_that_will_be_made_to_user(texts::ALL_PACKAGES_TO_REMOVE_FEDORA, texts::ALL_PACKAGES_TO_INSTALL_FEDORA_GNOME);
            utils::exec_installation("fedora","gnome");
            exit(0);

        },

        "--install-fedora-cinnamon" => {

            utils::show_the_changes_that_will_be_made_to_user(texts::ALL_PACKAGES_TO_REMOVE_FEDORA, texts::ALL_PACKAGES_TO_INSTALL_FEDORA_CINNAMON);
            utils::exec_installation("fedora","cinnamon");
            exit(0);

        },

        "--install-fedora-mate" => {

            utils::show_the_changes_that_will_be_made_to_user(texts::ALL_PACKAGES_TO_REMOVE_FEDORA, texts::ALL_PACKAGES_TO_INSTALL_FEDORA_MATE);
            utils::exec_installation("fedora","mate");
            exit(0);

        },

        "--install-fedora-kdeplasma" => {

            utils::show_the_changes_that_will_be_made_to_user(texts::ALL_PACKAGES_TO_REMOVE_FEDORA, texts::ALL_PACKAGES_TO_INSTALL_FEDORA_KDEPLASMA);
            utils::exec_installation("fedora","kdeplasma");
            exit(0);

        },

        "--install-fedora-bspwm" => {

            utils::show_the_changes_that_will_be_made_to_user(texts::ALL_PACKAGES_TO_REMOVE_FEDORA, texts::ALL_PACKAGES_TO_INSTALL_FEDORA_BSPWM);
            utils::exec_installation("fedora","bspwm");
            exit(0);

        },

        "--install-fedora-cutefish" => {

            utils::show_the_changes_that_will_be_made_to_user(texts::ALL_PACKAGES_TO_REMOVE_FEDORA, texts::ALL_PACKAGES_TO_INSTALL_FEDORA_CUTEFISH);
            utils::exec_installation("fedora","cutefish");
            exit(0);

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