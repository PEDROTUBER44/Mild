use std::{
    process::{
        Command,
        exit
    },

    io::{
        Write,
        stdout,
        stdin
    },

    path::Path,

    fs,

    io
};

use colored::Colorize;
use crate::texts;


pub fn remove_and_install_pkgs(remove_cmd: &str, install_cmd: &str, pkgs_to_remove: &str, pkgs_to_install: &str) {
    let remove_result = Command::new("sh").arg("-c").arg(format!("{} {}", remove_cmd, pkgs_to_remove)).status();
    
    match remove_result {
        Ok(_) => println!("Packages Removed {}", "Successfully".green().bold()),
        Err(_) => println!("{} To Remove Packages, {}: {:?}", "Error".red().bold(), "Error".red().bold(), remove_result.err())
    }

    let install_result = Command::new("sh").arg("-c").arg(format!("{} {}", install_cmd, pkgs_to_install)).status();
    
    match install_result {
        Ok(_) => println!("Packages Installed {}", "Successfully".green().bold()),
        Err(_) => println!("{} To Installing Packages, {}: {:?}", "Error".red().bold(), "Error".red().bold(), install_result.err())
    }
}

pub fn system_command(command: &str) {
    let exec_command = Command::new("sh").arg("-c").arg(format!("{}",command)).status();
    match exec_command {
        Ok(_) => println!("Command Executed {}", "Successfully".green().bold()),
        Err(e) => {
            println!("{}: {} To Execute Command", e, "Error".red().bold());
            exit(1);
        }
    }
}

pub fn write_text_in_file(text: &str, path: &str, delete_file: bool, as_root: bool) {
    match delete_file {
        true => {
            match as_root {
                true => {
                    let remove_file = Command::new("sudo").arg("sh").arg("-c").arg(format!("rm -r {}", path)).output();

                    match remove_file {
                        Ok(_) => println!("File Removed {}", "Successfully".green().bold()),
                        Err(_) => {
                            println!("{} Removing File: {}", "Error".red().bold(), path);
                            exit(1);
                        }
                    }

                    let cmd_root = Command::new("sudo").arg("sh").arg("-c").arg(format!("echo '{}' > {}", text, path)).output();
                    
                    match cmd_root {
                        Ok(_) => println!("File Written {}", "Successfully".green().bold()),
                        Err(_) => {
                            println!("{} Writing to File: {}", "Error".red().bold(), path);
                            exit(1);
                        }
                    }
                },
                false => {
                    let cpath = Path::new(&path);
                    let display = cpath.display();

                    match fs::remove_file(&cpath) {
                        Ok(()) => println!("File: {}, Removed {}", display, "Successfully".green().bold()),
                        Err(e) => {
                            if e.kind() != io::ErrorKind::NotFound {
                                println!("Could Not Remove File: {} {}: {}", display, "Error".red().bold(), e);
                                exit(1);
                            }
                        }
                    }

                    let mut file = match fs::File::create(&cpath) {
                        Ok(file) => file,
                        Err(e) => {
                            println!("Could Not Create File: {} {}: {}", display, "Error".red().bold(), e);
                            exit(0);
                        }
                    };

                    match file.write_all(text.as_bytes()) {
                        Ok(()) => println!("File: {}, Written {}", display, "Successfully".green().bold()),
                        Err(e) => {
                            println!("Could Not Write File: {} {}: {}", display, "Error".red().bold(), e);
                            exit(0);
                        }
                    };
                }
            }
        },
        false => {
            match as_root {
                true => {
                    let cmd_root = Command::new("sudo").arg("sh").arg("-c").arg(format!("echo '{}' > {}", text, path)).output();
                    
                    match cmd_root {
                        Ok(_) => println!("File Written {}", "Successfully".green().bold()),
                        Err(_) => {
                            println!("{} Writing to File: {}", "Error".red().bold(), path);
                            exit(1);
                        }
                    }
                },
                false => {
                    match fs::write(path, text) {
                        Ok(()) => println!("File: {}, Written {}", path, "Successfully".green().bold()),
                        Err(e) => {
                            println!("Could Not Write File: {} {}: {}", path, "Error".red().bold(), e);
                            exit(0);
                        }
                    };
                }
            }
        }
    }
}

pub fn remove_folder(folder: &str) {
    let remove_dir = fs::remove_dir(&folder);

    match remove_dir {
        Ok(_) => println!("Folder: {}, {} Removed", folder, "Successfully".green().bold()),
        Err(e) => {
            println!("Could Not Remove Folder: {} {}: {}", folder, "Error".red().bold(), e);
            exit(1);
        }
    };
}

pub fn install_flatpak_package_from_flathub(package_name: &str, id: &str) {
    let command_to_install = Command::new("sh").arg("-c").arg(format!("flatpak install flathub {} -y", id)).status();

    match command_to_install {
        Ok(_) => println!("Application: {} From Flathub {} Installed", package_name, "Successfully".green().bold()),
        Err(e) => {
            println!("{}: {} To Installing {} From Flathub", "Error".red().bold(), e, package_name);
            exit(1);
        }
    }
}

pub fn install_aur(url: &str, folder: &str) {
    system_command(&format!("git clone {}", url));
    system_command(&format!("cd {}", folder));
    system_command("makepkg -sicr");
    system_command("cd ..");
    remove_folder(folder);
}



pub fn install_system_and_utilities(all_packages_to_remove: &str, all_packages_to_install: &str, system: &str) {
    match &system[..] /* Configure System */ {
        "archlinux" => {
            write_text_in_file(texts::PACMAN_CONFIG_FILE, "/etc/pacman.conf", true, true);
            system_command("mv /usr/share/applications/avahi-discover.desktop /usr/share/applications/avahi-discover.backup");
            system_command("mv /usr/share/applications/bssh.desktop /usr/share/applications/bssh.backup");
            system_command("mv /usr/share/applications/bvnc.desktop /usr/share/applications/bvnc.backup");
            system_command("mv /usr/share/applications/nm-connection-editor.desktop /usr/share/applications/nm-connection-editor.backup");
            system_command("mv /usr/share/applications/qv4l2.desktop /usr/share/applications/qv4l2.backup");
            system_command("mv /usr/share/applications/qvidcap.desktop /usr/share/applications/qvidcap.backup");
        },

        "debian" => {
            write_text_in_file(texts::DEBIAN_CONFIG_FILE, "/etc/apt/sources.list", true, true);
            system_command("mv /usr/share/applications/vim.desktop /usr/share/applications/vim.backup");
        },

        "fedora" => {
            write_text_in_file(texts::DNF_CONFIG_FILE, "/etc/dnf/dnf.conf", true, true);
        },

        _ => {
            println!("{}", "Internal Error: System Not Found".red().bold());
            exit(1);
        }
    }

    loop {
        println!("These Packages Will Be Removed From Your System And After That Your System Will Be Updated:");
        println!("");
        println!("{}", all_packages_to_remove.red().bold());
        println!("");
        println!("");
        println!("The Following Packages Will Be Installed:");
        println!("");
        println!("{}", all_packages_to_install.green().bold());
        println!("");
        println!("");
        print!("Do You Want To Continue? ({}/{}): ","Y".green().bold(), "n".red().bold());

        stdout().flush().unwrap();
        let mut input: String = String::new();
        stdin().read_line(&mut input).expect("Error To Read User Input");

        match &input.trim().to_lowercase()[..] {
            "y" | "yes" | "" => {
                match &system[..] /* Remove Packages And Install New Enviroment */ {
                    "archlinux" => {
                        system_command(texts::DISABLE_DISPLAY_MANAGERS_CMD);
                        remove_and_install_pkgs("sudo pacman -Rsn --noconfirm", "sudo pacman -Syu --noconfirm", all_packages_to_remove, all_packages_to_install);
                        system_command(texts::INSTALL_UTILS_FOR_ARCHLINUX);
                        system_command(texts::INSTALL_FLATHUB);
                        system_command(texts::ENABLE_NETWORKMANAGER);
                    },
                    "debian" => {
                        system_command(texts::DISABLE_DISPLAY_MANAGERS_CMD);
                        remove_and_install_pkgs("sudo apt remove -y", "sudo apt install -y", all_packages_to_remove, all_packages_to_install);
                        system_command(texts::INSTALL_UTILS_FOR_DEBIAN);
                        system_command(texts::INSTALL_FLATHUB);
                        system_command(texts::ENABLE_NETWORKMANAGER);
                    },
                    "fedora" => {
                        system_command(texts::DISABLE_DISPLAY_MANAGERS_CMD);
                        remove_and_install_pkgs("sudo dnf remove -y", "sudo dnf install -y", all_packages_to_remove, all_packages_to_install);
                        system_command(texts::INSTALL_RPMFUSION_REPOSITORY);
                        system_command(texts::INSTALL_UTILS_FOR_FEDORA);
                        system_command(texts::INSTALL_FLATHUB);
                        system_command(texts::ENABLE_NETWORKMANAGER);
                    },
                    _ => {
                        println!("Internal {}: System Not Found", "Error".red().bold());
                        exit(1);
                    }
                }

                loop /* Internet Browser */ {
                    println!("Do You Want To Install An Internet Browser (Flatpak Format)?");
                    println!("");
                    println!("{} - {}","1".red().bold(), "Firefox".yellow().bold());
                    println!("{} - {}","2".red().bold(), "Chromium Browser".yellow().bold());
                    println!("{} - {}","3".red().bold(), "Brave".yellow().bold());
                    println!("{} - {}","4".red().bold(), "Microsoft Edge".yellow().bold());
                    println!("{} - {}","5".red().bold(), "None".yellow().bold());
                    println!("");
                    print!("Which Option Do You Want?: ");

                    stdout().flush().unwrap();
                    let mut option: String = String::new();
                    stdin().read_line(&mut option).expect("Error To Read User Input");
                    match &option.trim().to_lowercase()[..] {
                        "1" => {
                            install_flatpak_package_from_flathub("Firefox", "org.mozilla.firefox");
                            break;
                        },
                        "2" => {
                            install_flatpak_package_from_flathub("Chromium", "org.chromium.Chromium");
                            break;
                        },
                        "3" => {
                            install_flatpak_package_from_flathub("Brave", "com.brave.Browser");
                            break;
                        },
                        "4" => {
                            install_flatpak_package_from_flathub("Microsoft Edge", "com.microsoft.Edge");
                            break;
                        },
                        "5" => break,
                        _ => {
                            println!("{}","Please Enter A Valid Option!".red().bold());
                            system_command("clear");
                            continue;
                        }
                    }
                }

                loop /* Text Editor */ {
                    println!("Do You Want To Install An Text Editor (Flatpak Format)?");
                    println!("");
                    println!("{} - {} ({})","1".red().bold(), "Gnome Text Editor".yellow().bold(), "GTK".blue().bold());
                    println!("{} - {} ({})","2".red().bold(), "Kwrite".yellow().bold(), "QT".blue().bold());
                    println!("{} - {} ({})","3".red().bold(), "Mousepad".yellow().bold(), "GTK".blue().bold());
                    println!("{} - {} ({})","4".red().bold(), "Gedit".yellow().bold(), "GTK".blue().bold());
                    println!("{} - {}","5".red().bold(), "None".yellow().bold());
                    println!("");
                    print!("Which Option Do You Want?: ");

                    stdout().flush().unwrap();
                    let mut option: String = String::new();
                    stdin().read_line(&mut option).expect("Error To Read User Input");
                    match &option.trim().to_lowercase()[..] {
                        "1" => {
                            install_flatpak_package_from_flathub("Gnome Text Editor", "org.gnome.TextEditor");
                            break;
                        },
                        "2" => {
                            install_flatpak_package_from_flathub("Kwrite", "org.kde.kwrite");
                            break;
                        },
                        "3" => {
                            install_flatpak_package_from_flathub("Mousepad", "org.xfce.mousepad");
                            break;
                        },
                        "4" => {
                            install_flatpak_package_from_flathub("Gedit", "org.gnome.gedit");
                            break;
                        },
                        "5" => break,
                        _ => {
                            println!("{}","Please Enter A Valid Option!".red().bold());
                            system_command("clear");
                            continue;
                        }
                    }
                }

                loop /* Image Viewer */ {
                    println!("Do You Want To Install An Image Viewer (Flatpak Format)?");
                    println!("");
                    println!("{} - {} ({})","1".red().bold(), "EOG - Eye Of Gnome".yellow().bold(), "GTK".blue().bold());
                    println!("{} - {} ({})","2".red().bold(), "Gwenview".yellow().bold(), "QT".blue().bold());
                    println!("{} - {} ({})","3".red().bold(), "Ristretto".yellow().bold(), "GTK".blue().bold());
                    println!("{} - {} ({})","4".red().bold(), "Nomacs".yellow().bold(), "QT".blue().bold());
                    println!("{} - {}","5".red().bold(), "None".yellow().bold());
                    println!("");
                    print!("Which Option Do You Want?: ");

                    stdout().flush().unwrap();
                    let mut option: String = String::new();
                    stdin().read_line(&mut option).expect("Error To Read User Input");
                    match &option.trim().to_lowercase()[..] {
                        "1" => {
                            install_flatpak_package_from_flathub("EOG - Eye Of Gnome", "org.gnome.eog");
                            break;
                        },
                        "2" => {
                            install_flatpak_package_from_flathub("Gwenview", "org.kde.gwenview");
                            break;
                        },
                        "3" => {
                            install_flatpak_package_from_flathub("Ristretto", "org.xfce.ristretto");
                            break;
                        },
                        "4" => {
                            install_flatpak_package_from_flathub("Nomacs", "org.nomacs.ImageLounge");
                            break;
                        },
                        "5" => break,
                        _ => {
                            println!("{}","Please Enter A Valid Option!".red().bold());
                            system_command("clear");
                            continue;
                        }
                    }
                }

                loop /* Video Player */ {
                    println!("Do You Want To Install An Video Player (Flatpak Format)?");
                    println!("");
                    println!("{} - {} ({})","1".red().bold(), "Vlc".yellow().bold(), "QT".blue().bold());
                    println!("{} - {} ({})","2".red().bold(), "Totem".yellow().bold(), "GTK".blue().bold());
                    println!("{} - {} ({})","3".red().bold(), "Clapper".yellow().bold(), "GTK".blue().bold());
                    println!("{} - {} ({})","4".red().bold(), "SMPlayer".yellow().bold(), "GTK".blue().bold());
                    println!("{} - {} ({})","5".red().bold(), "Celluloid".yellow().bold(), "GTK".blue().bold());
                    println!("{} - {}","6".red().bold(), "None".yellow().bold());
                    println!("");
                    print!("Which Option Do You Want?: ");

                    stdout().flush().unwrap();
                    let mut option: String = String::new();
                    stdin().read_line(&mut option).expect("Error To Read User Input");
                    match &option.trim().to_lowercase()[..] {
                        "1" => {
                            install_flatpak_package_from_flathub("Vlc", "org.videolan.VLC");
                            break;
                        },
                        "2" => {
                            install_flatpak_package_from_flathub("Totem", "org.gnome.Totem");
                            break;
                        },
                        "3" => {
                            install_flatpak_package_from_flathub("Clapper", "com.github.rafostar.Clapper");
                            break;
                        },
                        "4" => {
                            install_flatpak_package_from_flathub("SMPlayer", "info.smplayer.SMPlayer");
                            break;
                        },
                        "5" => {
                            install_flatpak_package_from_flathub("Celluloid", "io.github.celluloid_player.Celluloid");
                            break;
                        },
                        "6" => break,
                        _ => {
                            println!("{}","Please Enter A Valid Option!".red().bold());
                            system_command("clear");
                            continue;
                        }
                    }
                }

                loop /* Document Viewer */ {
                    println!("Do You Want To Install An Document Viewer (Flatpak Format)?");
                    println!("");
                    println!("{} - {} ({})","1".red().bold(), "Evince".yellow().bold(), "GTK".blue().bold());
                    println!("{} - {} ({})","2".red().bold(), "Okular".yellow().bold(), "QT".blue().bold());
                    println!("{} - {}","3".red().bold(), "None".yellow().bold());
                    println!("");
                    print!("Which Option Do You Want?: ");

                    stdout().flush().unwrap();
                    let mut option: String = String::new();
                    stdin().read_line(&mut option).expect("Error To Read User Input");
                    match &option.trim().to_lowercase()[..] {
                        "1" => {
                            install_flatpak_package_from_flathub("Evince", "org.gnome.Evince");
                            break;
                        },
                        "2" => {
                            install_flatpak_package_from_flathub("Okular", "org.kde.okular");
                            break;
                        },
                        "3" => break,
                        _ => {
                            println!("{}","Please Enter A Valid Option!".red().bold());
                            system_command("clear");
                            continue;
                        }
                    }
                }

                loop /* Torrent Manager */ {
                    println!("Do You Want To Install An Torrent Manager (Flatpak Format)?");
                    println!("");
                    println!("{} - {} ({})","1".red().bold(), "Ktorrent".yellow().bold(), "QT".blue().bold());
                    println!("{} - {} ({})","2".red().bold(), "Transmission".yellow().bold(), "GTK".blue().bold());
                    println!("{} - {} ({})","3".red().bold(), "Fragments".yellow().bold(), "GTK".blue().bold());
                    println!("{} - {}","4".red().bold(), "None".yellow().bold());
                    println!("");
                    print!("Which Option Do You Want?: ");

                    stdout().flush().unwrap();
                    let mut option: String = String::new();
                    stdin().read_line(&mut option).expect("Error To Read User Input");
                    match &option.trim().to_lowercase()[..] {
                        "1" => {
                            install_flatpak_package_from_flathub("Ktorrent", "org.kde.ktorrent");
                            break;
                        },
                        "2" => {
                            install_flatpak_package_from_flathub("Transmission", "com.transmissionbt.Transmission");
                            break;
                        },
                        "3" => {
                            install_flatpak_package_from_flathub("Fragments", "de.haeckerfelix.Fragments");
                            break;
                        },
                        "4" => break,
                        _ => {
                            println!("{}","Please Enter A Valid Option!".red().bold());
                            system_command("clear");
                            continue;
                        }
                    }
                }

                loop /* Office Suite */ {
                    println!("Do You Want To Install An Office Suite (Flatpak Format)?");
                    println!("");
                    println!("{} - {} ({})","1".red().bold(), "Onlyoffice".yellow().bold(), "QT".blue().bold());
                    println!("{} - {} ({})","2".red().bold(), "WPS Office".yellow().bold(), "QT".blue().bold());
                    println!("{} - {} ({})","3".red().bold(), "LibreOffice".yellow().bold(), "GTK".blue().bold());
                    println!("{} - {}","4".red().bold(), "None".yellow().bold());
                    println!("");
                    print!("Which Option Do You Want?: ");

                    stdout().flush().unwrap();
                    let mut option: String = String::new();
                    stdin().read_line(&mut option).expect("Error To Read User Input");
                    match &option.trim().to_lowercase()[..] {
                        "1" => {
                            install_flatpak_package_from_flathub("Fragments", "de.haeckerfelix.Fragments");
                            break;
                        },
                        "2" => {
                            install_flatpak_package_from_flathub("WPS Office", "com.wps.Office");
                            break;
                        },
                        "3" => {
                            install_flatpak_package_from_flathub("LibreOffice", "org.libreoffice.LibreOffice");
                            break;
                        },
                        "4" => break,
                        _ => {
                            println!("{}","Please Enter A Valid Option!".red().bold());
                            system_command("clear");
                            continue;
                        }
                    }
                }

                loop /* Video Drivers */ {
                    println!("Do You Want To Install A Video Driver?");
                    println!("");
                    println!("{} - {}", "1".red().bold(), "Nvidia".yellow().bold());
                    println!("{} - {}", "2".red().bold(), "Intel".yellow().bold());
                    println!("{} - {}", "3".red().bold(), "Amd".yellow().bold());
                    println!("{} - {}", "4".red().bold(), "None".yellow().bold());
                    println!("");
                    print!("Which Option Do You Want?: ");

                    stdout().flush().unwrap();
                    let mut option: String = String::new();
                    stdin().read_line(&mut option).expect("Error To Read User Input");
                    match &option.trim().to_lowercase()[..] {
                        "1" => {
                            match &system[..] {
                                "archlinux" => {
                                    system_command("sudo pacman -Syu mesa nvidia nvidia-settings --noconfirm");
                                    break;
                                },
                                "debian" => {
                                    system_command("sudo apt install xorg mesa -y");
                                    break;
                                },
                                "fedora" => {
                                    system_command("sudo dnf install mesa -y");
                                    break;
                                },
                                _ => {
                                    println!("Internal {}: System Not Found", "Error".red().bold());
                                    exit(1);
                                }
                            }
                        },

                        "2" => {
                            match &system[..] {
                                "archlinux" => {
                                    system_command("pacman -S xf86-video-intel mesa libgl --noconfirm");
                                    break;
                                },
                                "debian" => {
                                    system_command("apt install xorg mesa -y");
                                    break;
                                },
                                "fedora" => {
                                    system_command("apt install @base-x -y");
                                    break;
                                },
                                _ => {
                                    println!("Internal {}: System Not Found", "Error".red().bold());
                                    exit(1);
                                }
                            }
                        },

                        "3" => {
                            match &system[..] {
                                "archlinux" => {
                                    system_command("sudo pacman -S xorg-server xorg-xinit xorg-apps xf86-video-intel mesa libgl xf86-video-amdgpu --noconfirm");
                                    break;
                                },
                                "debian" => {
                                    system_command("sudo apt install xorg mesa -y");
                                    break;
                                },
                                "fedora" => {
                                    system_command("sudo apt install @base-x -y");
                                    break;
                                },
                                _ => {
                                    println!("Internal {}: System Not Found", "Error".red().bold());
                                    exit(1);
                                }
                            }
                        },

                        "4" => break,

                        _ => {
                            println!("{}","Please Enter A Valid Option!".red().bold());
                            system_command("clear");
                            continue;
                        }
                    }
                }

                loop /* Printer Support */ {
                    println!("Do You Want To Install Printer Support?");
                    println!("");
                    println!("{} - {}", "1".red().bold(), "CUPS/Printer Manager".yellow().bold());
                    println!("{} - {}", "2".red().bold(), "CUPS/Printer Manager/Simple Scan".yellow().bold());
                    println!("{} - {}", "3".red().bold(), "None".yellow().bold());
                    println!("");
                    print!("Which Option Do You Want?: ");

                    stdout().flush().unwrap();
                    let mut option: String = String::new();
                    stdin().read_line(&mut option).expect("Error To Read User Input");
                    match &option.trim().to_lowercase()[..] {
                        "1" => {
                            match &system[..] {
                                "archlinux" => {
                                    system_command("sudo pacman -S cups system-config-printer --noconfirm");
                                    break;
                                },
                                "debian" => {
                                    system_command("sudo apt install cups system-config-printer -y");
                                    break;
                                },
                                "fedora" => {
                                    system_command("sudo dnf install cups system-config-printer -y");
                                    break;
                                },
                                _ => {
                                    println!("Internal {}: System Not Found", "Error".red().bold());
                                    exit(1);
                                }
                            }
                        },

                        "2" => {
                            match &system[..] {
                                "archlinux" => {
                                    system_command("sudo pacman -S cups system-config-printer simple-scan --noconfirm");
                                    break;
                                },
                                "debian" => {
                                    system_command("sudo apt install cups system-config-printer simple-scan -y");
                                    break;
                                },
                                "fedora" => {
                                    system_command("sudo dnf install cups system-config-printer simple-scan -y");
                                    break;
                                },
                                _ => {
                                    println!("Internal {}: System Not Found", "Error".red().bold());
                                    exit(1);
                                }
                            }
                        },

                        "3" => break,

                        _ => {
                            println!("{}","Please Enter A Valid Option!".red().bold());
                            system_command("sleep 4");
                            system_command("clear");
                            continue;
                        }
                    }
                }

                loop /* Basic Utilities And Compressed File Support */ {
                    println!("Do You Want To Install Basic Utilities (*Required)?");
                    println!("");
                    println!("{} - {}", "1".red().bold(), "Basic Utilities".yellow().bold());
                    println!("{} - {}", "2".red().bold(), "Basic Utilities/Compressed File Support/EXFAT File System Support".yellow().bold());
                    println!("");
                    print!("Which Option Do You Want?: ");

                    stdout().flush().unwrap();
                    let mut option: String = String::new();
                    stdin().read_line(&mut option).expect("Error To Read User Input");
                    match &option.trim().to_lowercase()[..] {
                        "1" => {
                            match &system[..] {
                                "archlinux" => {
                                    system_command("sudo pacman -S ffmpeg gst-plugins-ugly gst-plugins-good gst-plugins-base gst-plugins-bad gst-libav gstreamer networkmanager gvfs-mtp gvfs-goa gvfs-google --noconfirm");
                                    system_command(texts::ENABLE_NETWORKMANAGER);
                                    install_aur("https://aur.archlinux.org/preload.git", "preload/");
                                    system_command(texts::ENABLE_PRELOAD);
                                    break;
                                },
                                "debian" => {
                                    system_command("sudo apt install gstreamer1.0-plugins-base gstreamer1.0-plugins-good gstreamer1.0-plugins-ugly gstreamer1.0-plugins-bad ffmpeg sox twolame vorbis-tools lame faad mencoder sudo preload -y");
                                    system_command(texts::ENABLE_NETWORKMANAGER);
                                    system_command(texts::ENABLE_PRELOAD);
                                    break;
                                },
                                "fedora" => {
                                    system_command("dnf copr enable elxreno/preload -y");
                                    system_command(r#"dnf install @multimedia lame\* gstreamer1-plugins-{bad-\*,good-\*,base} gstreamer1-plugin-openh264 gstreamer1-libav gstreamer1-plugins-ugly gstreamer1-plugins-bad-free gstreamer1-plugins-bad-free gstreamer1-plugins-bad-freeworld gstreamer1-plugins-bad-free-extras ffmpeg preload fedora-workstation-backgrounds NetworkManager --exclude=gstreamer1-plugins-bad-free-devel --exclude=lame-devel -y"#);
                                    system_command("dnf install https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-37.noarch.rpm -y");
                                    system_command("dnf install https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-37.noarch.rpm -y");
                                    system_command(texts::ENABLE_NETWORKMANAGER);
                                    system_command(texts::ENABLE_PRELOAD);
                                    break;
                                },
                                _ => {
                                    println!("Internal {}: System Not Found", "Error".red().bold());
                                    exit(1);
                                }
                            }
                        },

                        "2" => {
                            match &system[..] {
                                "archlinux" => {
                                    system_command("sudo pacman -S ffmpeg gst-plugins-ugly gst-plugins-good gst-plugins-base gst-plugins-bad gst-libav gstreamer networkmanager gvfs-mtp gvfs-goa gvfs-google p7zip zip unzip unrar exfat-utils --noconfirm");
                                    system_command(texts::ENABLE_NETWORKMANAGER);
                                    install_aur("https://aur.archlinux.org/preload.git", "preload/");
                                    system_command(texts::ENABLE_PRELOAD);
                                    break;
                                },
                                "debian" => {
                                    system_command("apt install gstreamer1.0-plugins-base gstreamer1.0-plugins-good gstreamer1.0-plugins-ugly gstreamer1.0-plugins-bad ffmpeg sox twolame vorbis-tools lame faad mencoder sudo preload exfat-fuse exfat-utils p7zip-full zip unzip unrar-free -y");
                                    system_command(texts::ENABLE_NETWORKMANAGER);
                                    system_command(texts::ENABLE_PRELOAD);
                                    break;
                                },
                                "fedora" => {
                                    system_command("sudo dnf copr enable elxreno/preload -y");
                                    system_command(r#"sudo dnf install @multimedia lame\* gstreamer1-plugins-{bad-\*,good-\*,base} gstreamer1-plugin-openh264 gstreamer1-libav gstreamer1-plugins-ugly gstreamer1-plugins-bad-free gstreamer1-plugins-bad-free gstreamer1-plugins-bad-freeworld gstreamer1-plugins-bad-free-extras ffmpeg preload fedora-workstation-backgrounds NetworkManager unrar p7zip zip unzip --exclude=gstreamer1-plugins-bad-free-devel --exclude=lame-devel -y"#);
                                    system_command("sudo dnf install https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-37.noarch.rpm -y");
                                    system_command("sudo dnf install https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-37.noarch.rpm -y");
                                    system_command(texts::ENABLE_NETWORKMANAGER);
                                    system_command(texts::ENABLE_PRELOAD);
                                    break;
                                },
                                _ => {
                                    println!("Internal {}: System Not Found", "Error".red().bold());
                                    exit(1);
                                }
                            }
                        },

                        _ => {
                            println!("{}","Please Enter A Valid Option!".red().bold());
                            system_command("sleep 4");
                            system_command("clear");
                            continue;
                        }
                    }
                }

                loop /* Compressed File Manager */ {
                    println!("Do You Want To Install Compressed File Manager?");
                    println!("");
                    println!("{} - {} ({})", "1".red().bold(), "File Roller".yellow().bold(), "GTK".blue().bold());
                    println!("{} - {} ({})", "1".red().bold(), "Ark".yellow().bold(), "QT".blue().bold());
                    println!("{} - {}", "3".red().bold(), "None".yellow().bold());
                    println!("");
                    print!("Which Option Do You Want?: ");

                    stdout().flush().unwrap();
                    let mut option: String = String::new();
                    stdin().read_line(&mut option).expect("Error To Read User Input");
                    match &option.trim().to_lowercase()[..] {
                        "1" => {
                            install_flatpak_package_from_flathub("File Roller", "org.gnome.FileRoller");
                            break;
                        },
                        "2" => {
                            install_flatpak_package_from_flathub("Ark", "org.kde.ark");
                            break;
                        },
                        "3" => break,
                        _ => {
                            println!("{}","Please Enter A Valid Option!".red().bold());
                            system_command("sleep 4");
                            system_command("clear");
                            continue;
                        }
                    }
                }

                loop /* Bluetooth Support */ {
                    print!("Do You Want To Install {} {} ({}/{})", "Bluetooth".red().bold(), "Support".yellow().bold(), "y".green().bold(), "n".red().bold());
                    
                    stdout().flush().unwrap();
                    let mut option: String = String::new();
                    stdin().read_line(&mut option).expect("Error To Read User Input");
                    match &option.trim().to_lowercase()[..] {
                        "y" | "yes" => {
                            match &system[..] {
                                "archlinux" => {
                                    system_command("sudo pacman -S bluez bluedevil --noconfirm");
                                    system_command(texts::ENABLE_BLUETOOTH);
                                    break;
                                },
                                "debian" => {
                                    system_command("sudo apt install bluez -y");
                                    system_command(texts::ENABLE_BLUETOOTH);
                                    break;
                                },
                                "fedora" => {
                                    system_command("sudo dnf install bluez -y");
                                    system_command(texts::ENABLE_BLUETOOTH);
                                    break;
                                },
                                _ => {
                                    println!("Internal {}: System Not Found", "Error".red().bold());
                                    exit(1);
                                }
                            }
                        },

                        "n" | "no" => break,

                        _ => {
                            println!("{}","Please Enter A Valid Option!".red().bold());
                            system_command("sleep 4");
                            system_command("clear");
                            continue;
                        }
                    }
                }
                loop /* Terminal */ {
                    println!("Which Terminal Do You Want To Use (*Required)?");
                    println!("");
                    println!("{} - {}", "1".red().bold(), "Console".yellow().bold());
                    println!("{} - {}", "2".red().bold(), "Xfce4 Terminal".yellow().bold());
                    println!("{} - {}", "3".red().bold(), "Lxterminal".yellow().bold());
                    println!("{} - {}", "4".red().bold(), "Konsole".yellow().bold());
                    println!("{} - {}", "5".red().bold(), "Cutefish Terminal".yellow().bold());
                    println!("{} - {}", "6".red().bold(), "Qterminal".yellow().bold());
                    println!("{} - {}", "7".red().bold(), "Mate Terminal".yellow().bold());
                    println!("");
                    print!("Which Option Do You Want?: ");

                    stdout().flush().unwrap();
                    let mut option: String = String::new();
                    stdin().read_line(&mut option).expect("Error To Read User Input");
                    match &option.trim().to_lowercase()[..] {
                        "1" => {
                            match &system[..] {
                                "archlinux" => {
                                    system_command("pacman -S gnome-console --noconfirm");
                                    break;
                                },
                                "debian" => {
                                    system_command("sudo apt install gnome-console --no-install-requirements -y");
                                    break;
                                },
                                "fedora" => {
                                    system_command("sudo dnf install gnome-console -y");
                                    break;
                                },
                                _ => {
                                    println!("Internal {}: System Not Found", "Error".red().bold());
                                    exit(1);
                                }
                            }
                        },

                        "2" => {
                            match &system[..] {
                                "archlinux" => {
                                    system_command("sudo pacman -S xfce4-terminal --noconfirm");
                                    break;
                                },
                                "debian" => {
                                    system_command("sudo apt install xfce4-terminal --no-install-requirements -y");
                                    break;
                                },
                                "fedora" => {
                                    system_command("sudo dnf install xfce4-terminal -y");
                                    break;
                                },
                                _ => {
                                    println!("Internal {}: System Not Found", "Error".red().bold());
                                    exit(1);
                                }
                            }
                        },

                        "3" => {
                            match &system[..] {
                                "archlinux" => {
                                    system_command("sudo pacman -S lxterminal --noconfirm");
                                    break;
                                },
                                "debian" => {
                                    system_command("sudo apt install lxterminal --no-install-requirements -y");
                                    break;
                                },
                                "fedora" => {
                                    system_command("sudo dnf install lxterminal -y");
                                    break;
                                },
                                _ => {
                                    println!("Internal {}: System Not Found", "Error".red().bold());
                                    exit(1);
                                }
                            }
                        },

                        "4" => {
                            match &system[..] {
                                "archlinux" => {
                                    system_command("sudo pacman -S konsole --noconfirm");
                                    break;
                                },
                                "debian" => {
                                    system_command("sudo apt install konsole --no-install-requirements -y");
                                    break;
                                },
                                "fedora" => {
                                    system_command("sudo dnf install konsole -y");
                                    break;
                                },
                                _ => {
                                    println!("Internal {}: System Not Found", "Error".red().bold());
                                    exit(1);
                                }
                            }
                        },

                        "5" => {
                            match &system[..] {
                                "archlinux" => {
                                    system_command("sudo pacman -S cutefish-terminal --noconfirm");
                                    break;
                                },
                                "debian" => {
                                    system_command("clear");
                                    println!("Coming soon!!");
                                    system_command("sleep");
                                    continue;
                                },
                                "fedora" => {
                                    system_command("clear");
                                    println!("Coming soon!!");
                                    system_command("sleep");
                                    continue;
                                },
                                _ => {
                                    println!("Internal {}: System Not Found", "Error".red().bold());
                                    exit(1);
                                }
                            }
                        },

                        "6" => {
                            match &system[..] {
                                "archlinux" => {
                                    system_command("sudo pacman -S qterminal --noconfirm");
                                    break;
                                },
                                "debian" => {
                                    system_command("sudo apt install qterminal --no-install-requirements -y");
                                    break;
                                },
                                "fedora" => {
                                    system_command("sudo dnf install qterminal -y");
                                    break;
                                },
                                _ => {
                                    println!("Internal {}: System Not Found", "Error".red().bold());
                                    exit(1);
                                }
                            }
                        },

                        "7" => {
                            match &system[..] {
                                "archlinux" => {
                                    system_command("sudo pacman -S mate-terminal --noconfirm");
                                    break;
                                },
                                "debian" => {
                                    system_command("sudo apt install mate-terminal -y");
                                    break;
                                },
                                "fedora" => {
                                    system_command("sudo dnf install mate-terminal -y");
                                    break;
                                },
                                _ => {
                                    println!("Internal {}: System Not Found", "Error".red().bold());
                                    exit(1);
                                }
                            }
                        },

                        _ => {
                            println!("{}","Please Enter A Valid Option!".red().bold());
                            system_command("sleep 4");
                            system_command("clear");
                            continue;
                        }
                    }
                }

                loop /* Display Manager */ {
                    println!("Which Display Manager Do You Want To Use (*Required)?");
                    println!("");
                    println!("{} - {}", "1".red().bold(), "Gdm".yellow().bold());
                    println!("{} - {}", "2".red().bold(), "Lightdm".yellow().bold());
                    println!("{} - {}", "3".red().bold(), "SDDM".yellow().bold());
                    println!("{} - {}", "4".red().bold(), "LXDM".yellow().bold());
                    println!("");
                    print!("Which Option Do You Want?: ");

                    stdout().flush().unwrap();
                    let mut option: String = String::new();
                    stdin().read_line(&mut option).expect("Error To Read User Input");
                    match &option.trim().to_lowercase()[..] {
                        "1" => {
                            match &system[..] {
                                "archlinux" => {
                                    system_command("sudo pacman -S gdm --noconfirm");
                                    system_command(texts::ENABLE_GDM);
                                    break;
                                },
                                "debian" => {
                                    system_command("sudo apt install gdm3 --no-install-requirements -y");
                                    system_command(texts::ENABLE_GDM);
                                    break;
                                },
                                "fedora" => {
                                    system_command("sudo dnf install gdm -y");
                                    system_command(texts::ENABLE_GDM);
                                    system_command(texts::ENABLE_GRAPHICAL_INITIALIZATION);
                                    break;
                                },
                                _ => {
                                    println!("Internal {}: System Not Found", "Error".red().bold());
                                    exit(1);
                                }
                            }
                        },

                        "2" => {
                            match &system[..] {
                                "archlinux" => {
                                    system_command("sudo pacman -S lightdm lightdm-gtk-greeter --noconfirm");
                                    system_command(texts::ENABLE_LIGHTDM);
                                    break;
                                },
                                "debian" => {
                                    system_command("sudo apt install lightdm lightdm-gtk-greeter --no-install-requirements -y");
                                    system_command(texts::ENABLE_LIGHTDM);
                                    break;
                                },
                                "fedora" => {
                                    system_command("sudo dnf install lightdm lightdm-gtk-greeter -y");
                                    system_command(texts::ENABLE_LIGHTDM);
                                    system_command(texts::ENABLE_GRAPHICAL_INITIALIZATION);
                                    break;
                                },
                                _ => {
                                    println!("Internal {}: System Not Found", "Error".red().bold());
                                    exit(1);
                                }
                            }
                        },

                        "3" => {
                            match &system[..] {
                                "archlinux" => {
                                    system_command("sudo pacman -S sddm --noconfirm");
                                    system_command(texts::ENABLE_SDDM);
                                    break;
                                },
                                "debian" => {
                                    system_command("sudo apt install sddm --no-install-requirements -y");
                                    system_command(texts::ENABLE_SDDM);
                                    break;
                                },
                                "fedora" => {
                                    system_command("sudo dnf install sddm -y");
                                    system_command(texts::ENABLE_SDDM);
                                    system_command(texts::ENABLE_GRAPHICAL_INITIALIZATION);
                                    break;
                                },
                                _ => {
                                    println!("Internal {}: System Not Found", "Error".red().bold());
                                    exit(1);
                                }
                            }
                        },

                        "4" => {
                            match &system[..] {
                                "archlinux" => {
                                    system_command("sudo pacman -S lxdm --noconfirm");
                                    system_command(texts::ENABLE_LXDM);
                                    break;
                                },
                                "debian" => {
                                    system_command("sudo apt install lxdm --no-install-requirements -y");
                                    system_command(texts::ENABLE_LXDM);
                                    break;
                                },
                                "fedora" => {
                                    system_command("sudo dnf install lxdm -y");
                                    system_command(texts::ENABLE_LXDM);
                                    system_command(texts::ENABLE_GRAPHICAL_INITIALIZATION);
                                    break;
                                },
                                _ => {
                                    println!("Internal {}: System Not Found", "Error".red().bold());
                                    exit(1);
                                }
                            }
                        },

                        _ => {
                            println!("{}","Please Enter A Valid Option!".red().bold());
                            system_command("sleep 4");
                            system_command("clear");
                            continue;
                        }
                    }
                }
            },

            "n" | "no" => {
                system_command("clear");
                println!("Aborted installation");
                exit(0);
            },
            
            _ => continue
        }

        break;
    }
}