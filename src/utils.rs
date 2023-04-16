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



pub fn remove_and_install_pkgs(remove_command: &str, install_command: &str) {
    let remove_result = Command::new("sh").arg("-c").arg(remove_command).status();
    
    match remove_result {
        Ok(_) => println!("Packages Removed {}", "Successfully".green().bold()),
        Err(_) => println!("{} To Remove Packages, {}: {:?}", "Error".red().bold(), "Error".red().bold(), remove_result.err())
    }

    let install_result = Command::new("sh").arg("-c").arg(install_command).status();
    
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

pub fn error_system_not_found() {
    println!("Internal {}: System Not Found", "Error".red().bold());
    exit(1);
}

pub fn invalid_option_selected_error() {
    system_command("clear");
    println!("{}","Please Enter A Valid Option!".red().bold());
    system_command("sleep 5");
    system_command("clear");
}

pub fn remove_repeated_words(text: &str) -> String {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut sentence_without_repeated_words: Vec<&str> = Vec::new();

    for word in words {
        if !sentence_without_repeated_words.contains(&word) {
            sentence_without_repeated_words.push(word);
        }
    }

    sentence_without_repeated_words.join(" ")
}

pub fn install_system_and_utilities(all_packages_to_remove: String, all_packages_to_install: &str, system: &str) {

    pub const DISABLE_DISPLAY_MANAGERS_CMD: &str = "sudo systemctl disable gdm -f && sudo systemctl disable lightdm -f && sudo systemctl disable sddm -f && sudo systemctl disable lxdm -f";
    pub const INSTALL_FLATHUB: &str = "flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo";
    pub const ENABLE_NETWORKMANAGER: &str = "sudo systemctl enable NetworkManager -f";
    pub const ENABLE_GDM: &str = "sudo systemctl enable gdm -f";
    pub const ENABLE_LIGHTDM: &str = "sudo systemctl enable lightdm -f";
    pub const ENABLE_SDDM: &str = "sudo systemctl enable sddm -f";
    pub const ENABLE_LXDM: &str = "sudo systemctl enable lxdm -f";
    pub const ENABLE_PRELOAD: &str = "sudo systemctl enable preload -f";
    pub const ENABLE_BLUETOOTH: &str = "sudo systemctl enable bluetooth -f";
    pub const ENABLE_GRAPHICAL_INITIALIZATION: &str = "sudo systemctl set-default graphical.target";

    match &system[..] /* Configure System */ {
        "archlinux" => {
            system_command("clear");
            println!("{} The Multilib Repositories", "Uncomment".bold().yellow());
            system_command("sleep 4");
            system_command("sudo nano /etc/pacman.conf");
            system_command("sudo pacman -Sy --noconfirm"); /* Update Repositories list */
            system_command("sudo mv /usr/share/applications/avahi-discover.desktop /usr/share/applications/avahi-discover.backup");
            system_command("sudo mv /usr/share/applications/bssh.desktop /usr/share/applications/bssh.backup");
            system_command("sudo mv /usr/share/applications/bvnc.desktop /usr/share/applications/bvnc.backup");
            system_command("sudo mv /usr/share/applications/nm-connection-editor.desktop /usr/share/applications/nm-connection-editor.backup");
            system_command("sudo mv /usr/share/applications/qv4l2.desktop /usr/share/applications/qv4l2.backup");
            system_command("sudo mv /usr/share/applications/qvidcap.desktop /usr/share/applications/qvidcap.backup");
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
                        system_command(DISABLE_DISPLAY_MANAGERS_CMD); // This command will disable all display managers on the system
                        remove_and_install_pkgs(&format!("sudo pacman -Rsn {} --noconfirm",all_packages_to_remove), &format!("sudo pacman -Syu {} --noconfirm", all_packages_to_install)); // This line of code will remove almost all packages for each graphical interface and after that it will install the graphical environment packages that the user wants
                        // List Of Basic System Utilities That Will Be Installed:
                        //
                        // Miscellaneous:
                        //
                        // - flatpak: Linux application sandboxing and distribution framework (formerly xdg-app).
                        // - networkmanager: Network connection manager and user applications.
                        // - xdg-user-dirs: Manage user directories like ~/Desktop and ~/Music.
                        // - exfat-utils: Utilities for exFAT file system.
                        //
                        //
                        // Window System:
                        // 
                        // - xorg: Xorg X server (packages).
                        // - xorg-server: Xorg X server.
                        //
                        //
                        // Compressed file handlers:
                        //
                        // - p7zip: Command-line file archiver with high compression ratio.
                        // - zip: Compressor/archiver for creating and modifying zipfiles.
                        // - unzip: For extracting and viewing files in .zip archives.
                        // - unrar: The RAR uncompression program.
                        //
                        //
                        // GVFS:
                        //
                        // - gvfs-mtp: Virtual filesystem implementation for GIO (MTP backend; Android, media player).
                        // - gvfs-goa: Virtual filesystem implementation for GIO (Gnome Online Accounts backend; cloud storage).
                        // - gvfs-google: Virtual filesystem implementation for GIO (Google Drive backend).
                        //
                        //
                        // Codecs:
                        //
                        // - ffmpeg: Complete solution to record, convert and stream audio and video.
                        // - gst-plugins-ugly: Multimedia graph framework - ugly plugins.
                        // - gst-plugins-good: Multimedia graph framework - good plugins.
                        // - gst-plugins-base: gst-plugins-baseMultimedia graph framework - base plugins.
                        // - gst-plugins-bad: Multimedia graph framework - bad plugins.
                        // - gst-libav: Multimedia graph framework - libav plugin.
                        // - gstreamer: Multimedia graph framework - core.
                        // - a52dec: A free library for decoding ATSC A/52 streams.
                        // - faac: Freeware Advanced Audio Coder.
                        // - faad2: Freeware Advanced Audio (AAC) Decoder.
                        // - flac: Free Lossless Audio Codec.
                        // - jasper: Software-based implementation of the codec specified in the emerging JPEG-2000 Part-1 standard.
                        // - lame: A high quality MPEG Audio Layer III (MP3) encoder.
                        // - libdca: Free library for decoding DTS Coherent Acoustics streams.
                        // - libdv: The Quasar DV codec (libdv) is a software codec for DV video.
                        // - libmad: A high-quality MPEG audio decoder.
                        // - libmpeg2: Library for decoding MPEG-1 and MPEG-2 video streams.
                        // - libtheora: An open video codec developed by the Xiph.org.
                        // - libvorbis: Reference implementation of the Ogg Vorbis audio format.
                        // - libxv: X11 Video extension library.
                        // - opus: Totally open, royalty-free, highly versatile audio codec.
                        // - wavpack: Audio compression format with lossless, lossy and hybrid compression modes
                        // - x264: Open Source H264/AVC video encoder.
                        // - xvidcore: XviD is an open source MPEG-4 video codec.
                        //
                        //
                        // https://archlinux.org
                        //
                        system_command("sudo pacman -S flatpak xorg xorg-server xdg-user-dirs networkmanager gvfs-mtp gvfs-goa gvfs-google exfat-utils p7zip zip unzip unrar ffmpeg gst-plugins-ugly gst-plugins-good gst-plugins-base gst-plugins-bad gst-libav gstreamer a52dec faac faad2 flac jasper lame libdca libdv libmad libmpeg2 libtheora libvorbis libxv opus wavpack x264 xvidcore --noconfirm");
                        system_command(INSTALL_FLATHUB); // This command installs the flathub flatpak package repository
                        system_command(ENABLE_NETWORKMANAGER); // This command enable the networkmanager daemon
                        /*Fix*/ // Add preload install
                    },
                    "debian" => {
                        system_command(DISABLE_DISPLAY_MANAGERS_CMD); // This command will disable all display managers on the system
                        remove_and_install_pkgs(&format!("sudo apt remove {} -y",all_packages_to_remove), &format!("sudo apt install {} -y", all_packages_to_install)); // This line of code will remove almost all packages for each graphical interface and after that it will install the graphical environment packages that the user wants
                        // List Of Basic System Utilities That Will Be Installed:
                        //
                        // Miscellaneous:
                        //
                        // - flatpak: Application deployment framework for desktop apps.
                        // - sudo: Provide limited super user privileges to specific users.
                        // - network-manager: network management framework (daemon and userspace tools).
                        // - pulseaudio: PulseAudio sound server.
                        // - exfat-utils: utilities to create, check, label and dump exFAT filesystem.
                        // - xdg-user-dirs: tool to manage well known user directories.
                        //
                        //
                        // Window system:
                        //
                        // - xorg: X.Org X Window System.
                        //
                        //
                        // Compressed file handlers:
                        //
                        // - zip: Archiver for .zip files.
                        // - unzip: De-archiver for .zip files.
                        // - unrar-free: Unarchiver for .rar files.
                        // - p7zip-full: 7z and 7za file archivers with high compression ratio.
                        //
                        //
                        // GVFS:
                        //
                        // - gvfs: userspace virtual filesystem - GIO module.
                        //
                        //
                        // Codecs:
                        //
                        // - gstreamer1.0-plugins-base: GStreamer plugins from the "base" set.
                        // - gstreamer1.0-plugins-good: GStreamer plugins from the "good" set.
                        // - gstreamer1.0-plugins-ugly: GStreamer plugins from the "ugly" set.
                        // - gstreamer1.0-plugins-bad: GStreamer plugins from the "bad" set.
                        // - ffmpeg: Tools for transcoding, streaming and playing of multimedia files
                        // - sox: Swiss army knife of sound processing.
                        // - twolame: MPEG Audio Layer 2 encoder (command line frontend).
                        // - vorbis-tools: several Ogg Vorbis tools.
                        // - lame: MP3 encoding library (frontend).
                        // - faad: freeware Advanced Audio Decoder player.
                        //
                        //
                        // https://packages.debian.org/en/
                        //
                        system_command("sudo apt install flatpak sudo zip unzip unrar-free xdg-user-dirs network-manager xorg gvfs pulseaudio exfat-utils p7zip-full gstreamer1.0-plugins-base gstreamer1.0-plugins-good gstreamer1.0-plugins-ugly gstreamer1.0-plugins-bad ffmpeg sox twolame vorbis-tools lame faad -y");
                        system_command(INSTALL_FLATHUB); // This command installs the flathub flatpak package repository
                        system_command(ENABLE_NETWORKMANAGER); // This command enable the networkmanager daemon
                    },
                    "fedora" => {
                        system_command(DISABLE_DISPLAY_MANAGERS_CMD); // This command will disable all display managers on the system
                        remove_and_install_pkgs(&format!("sudo dnf remove {} -y",all_packages_to_remove), &format!("sudo dnf install {} -y", all_packages_to_install)); // This line of code will remove almost all packages for each graphical interface and after that it will install the graphical environment packages that the user wants
                        system_command("sudo dnf install https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-37.noarch.rpm https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-37.noarch.rpm -y"); // Command to install free and non-free rpmfusion repositories for fedora 37
                        // List Of Basic System Utilities That Will Be Installed:
                        //
                        // Miscellaneous:
                        //
                        // - flatpak: Application deployment framework for desktop apps.
                        // - network-manager-applet: A network control and status applet for NetworkManager.
                        // - exfat-utils: Utilities for exFAT file system.
                        // - NetworkManager: Network connection manager and user applications.
                        // -
                        //
                        //
                        // Window System:
                        // 
                        // - @base-x: Xorg package set.
                        //
                        //
                        // Compressed file handlers:
                        //
                        // - unrar: Utility for extracting, testing and viewing RAR archives.
                        // - p7zip: Very high compression ratio file archiver.
                        // - zip: A file compression and packaging utility compatible with PKZIP.
                        // - unzip: A utility for unpacking zip files.
                        //
                        //
                        // GVFS:
                        //
                        // - gvfs-mtp: MTP support for gvfs.
                        // - gvfs-goa: GOA support for gvfs.
                        //
                        //
                        // Codecs:
                        //
                        // - @multimedia: Multimedia codecs packs set.
                        // - lame: Free MP3 audio compressor
                        // - gstreamer1-plugins-ugly: GStreamer 1.0 streaming media framework "ugly" plug-ins.
                        // - ffmpeg: Digital VCR and streaming server.
                        // - gstreamer1-plugins-{bad-\*,good-\*,base}: Various gstreamer1 codec packs.
                        // - gstreamer1-plugin-openh264: GStreamer H.264 plugin
                        // - gstreamer1-libav: GStreamer 1.0 libav-based plug-ins.
                        //
                        //
                        // Informations From Fedora Info Command And RpmFusion Website
                        //
                        system_command(r#"sudo dnf install flatpak @base-x @multimedia network-manager-applet unrar p7zip zip unzip NetworkManager exfat-utils lame gvfs-mtp gvfs-goa gstreamer1-plugins-ugly ffmpeg gstreamer1-plugins-{bad-\*,good-\*,base} gstreamer1-plugin-openh264 gstreamer1-libav --exclude=gstreamer1-plugins-bad-free-devel --exclude=lame-devel --skip-broken -y"#);
                        system_command(INSTALL_FLATHUB); // This command installs the flathub flatpak package repository
                        system_command(ENABLE_NETWORKMANAGER); // This command enable the networkmanager daemon
                    },
                    _ => {
                        error_system_not_found();
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
                            invalid_option_selected_error();
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
                            invalid_option_selected_error();
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
                            invalid_option_selected_error();
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
                            invalid_option_selected_error();
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
                            invalid_option_selected_error();
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
                            invalid_option_selected_error();
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
                            install_flatpak_package_from_flathub("Onlyoffice", "org.onlyoffice.desktopeditors");
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
                            invalid_option_selected_error();
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
                                    error_system_not_found();
                                }
                            }
                        },

                        "2" => {
                            match &system[..] {
                                "archlinux" => {
                                    system_command("sudo pacman -S xf86-video-intel mesa libgl --noconfirm");
                                    break;
                                },
                                "debian" => {
                                    system_command("sudo apt install xorg mesa -y");
                                    break;
                                },
                                "fedora" => {
                                    system_command("sudo dnf install @base-x -y");
                                    break;
                                },
                                _ => {
                                    error_system_not_found();
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
                                    error_system_not_found();
                                }
                            }
                        },

                        _ => {
                            invalid_option_selected_error();
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
                                    error_system_not_found();
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
                                    error_system_not_found();
                                }
                            }
                        },

                        "3" => break,

                        _ => {
                            invalid_option_selected_error();
                            continue;
                        }
                    }
                }

                loop /* Compressed File Manager */ {
                    println!("Do You Want To Install Compressed File Manager?");
                    println!("");
                    println!("{} - {} ({})", "1".red().bold(), "File Roller".yellow().bold(), "GTK".blue().bold());
                    println!("{} - {} ({})", "2".red().bold(), "Ark".yellow().bold(), "QT".blue().bold());
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
                            invalid_option_selected_error();
                            continue;
                        }
                    }
                }

                loop /* Bluetooth Support */ {
                    print!("Do You Want To Install {} {} ({}/{}): ", "Bluetooth".red().bold(), "Support".yellow().bold(), "y".green().bold(), "n".red().bold());
                    
                    stdout().flush().unwrap();
                    let mut option: String = String::new();
                    stdin().read_line(&mut option).expect("Error To Read User Input");
                    match &option.trim().to_lowercase()[..] {
                        "y" | "yes" => {
                            match &system[..] {
                                "archlinux" => {
                                    system_command("sudo pacman -S bluez bluedevil --noconfirm");
                                    system_command(ENABLE_BLUETOOTH);
                                    break;
                                },
                                "debian" => {
                                    system_command("sudo apt install bluez -y");
                                    system_command(ENABLE_BLUETOOTH);
                                    break;
                                },
                                "fedora" => {
                                    system_command("sudo dnf install bluez -y");
                                    system_command(ENABLE_BLUETOOTH);
                                    break;
                                },
                                _ => {
                                    error_system_not_found();
                                }
                            }
                        },

                        "n" | "no" => break,

                        _ => {
                            invalid_option_selected_error();
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
                                    system_command("sudo pacman -S gnome-console --noconfirm");
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
                                    error_system_not_found();
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
                                    error_system_not_found();
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
                                    error_system_not_found();
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
                                    error_system_not_found();
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
                                    error_system_not_found();
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
                                    error_system_not_found();
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
                                    error_system_not_found();
                                }
                            }
                        },

                        _ => {
                            invalid_option_selected_error();
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
                                    system_command(ENABLE_GDM);
                                    break;
                                },
                                "debian" => {
                                    system_command("sudo apt install gdm3 --no-install-requirements -y");
                                    system_command(ENABLE_GDM);
                                    break;
                                },
                                "fedora" => {
                                    system_command("sudo dnf install gdm -y");
                                    system_command(ENABLE_GDM);
                                    system_command(ENABLE_GRAPHICAL_INITIALIZATION);
                                    break;
                                },
                                _ => {
                                    error_system_not_found();
                                }
                            }
                        },

                        "2" => {
                            match &system[..] {
                                "archlinux" => {
                                    system_command("sudo pacman -S lightdm lightdm-gtk-greeter --noconfirm");
                                    system_command(ENABLE_LIGHTDM);
                                    break;
                                },
                                "debian" => {
                                    system_command("sudo apt install lightdm lightdm-gtk-greeter --no-install-requirements -y");
                                    system_command(ENABLE_LIGHTDM);
                                    break;
                                },
                                "fedora" => {
                                    system_command("sudo dnf install lightdm lightdm-gtk-greeter -y");
                                    system_command(ENABLE_LIGHTDM);
                                    system_command(ENABLE_GRAPHICAL_INITIALIZATION);
                                    break;
                                },
                                _ => {
                                    error_system_not_found();
                                }
                            }
                        },

                        "3" => {
                            match &system[..] {
                                "archlinux" => {
                                    system_command("sudo pacman -S sddm --noconfirm");
                                    system_command(ENABLE_SDDM);
                                    break;
                                },
                                "debian" => {
                                    system_command("sudo apt install sddm --no-install-requirements -y");
                                    system_command(ENABLE_SDDM);
                                    break;
                                },
                                "fedora" => {
                                    system_command("sudo dnf install sddm -y");
                                    system_command(ENABLE_SDDM);
                                    system_command(ENABLE_GRAPHICAL_INITIALIZATION);
                                    break;
                                },
                                _ => {
                                    error_system_not_found();
                                }
                            }
                        },

                        "4" => {
                            match &system[..] {
                                "archlinux" => {
                                    system_command("sudo pacman -S lxdm --noconfirm");
                                    system_command(ENABLE_LXDM);
                                    break;
                                },
                                "debian" => {
                                    system_command("sudo apt install lxdm --no-install-requirements -y");
                                    system_command(ENABLE_LXDM);
                                    break;
                                },
                                "fedora" => {
                                    system_command("sudo dnf install lxdm -y");
                                    system_command(ENABLE_LXDM);
                                    system_command(ENABLE_GRAPHICAL_INITIALIZATION);
                                    break;
                                },
                                _ => {
                                    error_system_not_found();
                                }
                            }
                        },

                        _ => {
                            invalid_option_selected_error();
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