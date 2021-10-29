# MILD - Minimal Install Linux Desktop
MILD é um instalador em modo texto, simples e direto que tem o intuito de instalar um "D.E."(Desktop Enviroment) com só os apps realmente necessários de cada ambiente gráfico. Veja abaixo como instalar em sua distro preferida:

## Fedora 34

Primeiramente baixe do site oficial a iso netinstall do fedora 34, após isso faça um pendrive bootavel, com a iso netinstall do fedora 34 e configure na BIOS, para dar boot pelo pendrive. Após isso você verá essa tela e nela você deverá selecionar "Test this media & install Fedora 34", para verificar se a iso que está no pendrive está corrompida e instalar o Fedora 34:

![](imagens/iniciofedora34.webp)


Após isso você verá essa tela e nela selecione o seu idioma, e após isso clique em continuar:

![](imagens/idiomafedora34.webp)


Depois disso você verá a tela abaixo e nela selecione "Destino da instalação", e depois selecione o disco que será instalado como também particionado para a instalação do Fedora 34, **Atenção não nos responsabilizamos por eventuais perdas de dados**, e recomendamos que faça um backup dos seus dados em uma unidade à parte e de preferência desconectado do seu pc pelo menos durante a instalação, e também marque a caixinha "Personalização avançada ( Blivet GUI )", e depois clique em "Pronto":

![](imagens/configsfedora34.webp)


Logo após ele abrirá a tela de particionamento do disco, e nela selecione o espaço livre e clique em "+" para criar uma partição.

![](imagens/discofedora34.webp)

Em sistemas **UEFI** faça as seguintes partiçoẽs (Lembrando que a ordem não importa) e depois clique em "Pronto":

Ordem | Tipo | Tamanho
------|------|--------
  1°  | efi  | 500MB  
  2°  | swap | O tamanho da sua memória ram.    
  3°  | root | 70GB   
  4°  | home | Todo o resto*

![](imagens/discouefifedora34.webp)

Em sistemas **Legacy** faça as seguintes partiçoẽs (Lembrando que a ordem não importa) e depois clique em "Pronto":

Ordem | Tipo | Tamanho
------|------|--------  
  1°  | swap | O tamanho da sua memória ram. 
  2°  | root | 70GB   
  3°  | home | Todo o resto*

![](imagens/discolegacyfedora34.webp)


Depois você voltará a essa tela, nela você poderá "Rede e nome do host" para configurar o wifi caso esteja em um notebook ou queira trocar o nome do host que por padrão é "fedora" e também poderá selecionar o layout do teclado que por padrão já é determinado automaticamente, clicando em "Teclado" caso o teclado não estiver correto.

![](imagens/redefedora34.webp)


Após essas configurações clique em "Seleção de programas" e marque apenas "Instalação mínima" como a imagem abaixo e caso deseje utilizar wifi, marque também "Submódulos comuns do NetworkManager":

![](imagens/1-pacotesfedora34.webp)


E caso seu PC não tenha wifi selecione apenas "Instalação mínima":

![](imagens/2-pacotesfedora34.webp)


Agora clique em "Senha do root" para criar a senha do usuário root, e clique em "Pronto":

![](imagens/rootfedora34.webp)


Agora clique em "Criação de usuário" para criar a senha e o usuário, e clique em "Pronto":

![](imagens/usuariofedora34.webp)


Depois disso clique em "Iniciar a instalação" e aguarde alguns minutos, lembrando que o tempo pode variar dependendo da potência do seu pc e a velocidade da sua internet. Após o término reinicie o seu pc e retire o pendrive.


### Pós instalação

Agora você cairá na tela do terminal, para terminar a instalação do sistema, primeiramente logue-se como usuário **root** após isso, digite os seguintes comandos:

**O 1° para instalar o git:**

  dnf install git -y


**O 2° para clonar esse repositório:**

  git clone https://github.com/PEDROTUBER44/mild.git


**O 3° para entrar dentro da pasta do repositório clonado:**

  cd mild


**O 4° para dar permissão de execução para o arquivo de instalação do mild:**

  chmod +x install.sh


**O 5° para instalar o mild em seu PC:**

  ./install.sh

****


Agora escolha qual ambiente gráfico que será instalado em seu PC:


### Lxde

  mild --install-fedora-lxde

![](imagens/fedora-lxde.webp)

---


### Lxqt

  mild --install-fedora-lxqt

![](imagens/fedora-lxqt.webp)

---

### Xfce

  mild --install-fedora-xfce

![](imagens/fedora-xfce.webp)

---

### Gnome

  mild --install-fedora-gnome

![](imagens/fedora-gnome.webp)

---


### Mate

  mild --install-fedora-mate

![](imagens/fedora-mate.webp)

---

### Kde Plasma

  mild --install-fedora-kdeplasma

![](imagens/fedora-kdeplasma.webp)

---

### Cinnamon

  mild --install-fedora-cinnamon

![](imagens/fedora-cinnamon.webp)

Após isso você terá um ambiente desktop clean sem aplicativos redundantes no seu **Fedora 34 !**

****


## Debian 11

Primeiramente baixe do site oficial a iso netinstall do debian 11, após isso faça um pendrive bootavel, com a iso netinstall do debian 11 e configure na BIOS, para dar boot pelo pendrive. Após isso você verá essa tela e nela você deverá selecionar "Graphical Install" para instalar o debian de forma gráfica:

![](imagens/iniciodebian11.webp)


Depois disso selecione qual será o idioma do sistema, que no meu caso eu optei por "Portuguese (Brasil)":

![](imagens/idiomadebian11.webp)


Agora selecione a seu local:

![](imagens/localidadedebian11.webp)


Selecione agora o seu layout do teclado:

![](imagens/layoutdotecladodebian11.webp)


Dê agora um nome para seu PC na rede:

![](imagens/hostnamedebian11.webp)


Nessa parte clique em "Continuar":

![](imagens/dominiodebian11.webp)


Agora escreva a senha do root:

![](imagens/rootdebian11.webp)


Escreva o seu nome que será usado para seu usuário (Sem espaços):

![](imagens/contadebian11.webp)


Escreva o nome do usuário novamente (Sem espaços):

![](imagens/usuariodebian11.webp)


Escreva a senha do seu usuário:

![](imagens/senhausuariodebian11.webp)


Selecione o fuso horário mais perto de você (No meu caso estou mais perto de São Paulo):

![](imagens/horariodebian11.webp) 


Agora particione o seu disco de acordo com sua bios:

Em sistemas **UEFI** faça as seguintes partiçoẽs (Lembrando que a ordem não importa) e e selecione "Finalizar o particionamento e escrever as mudanças no disco", depois clique em "Continuar":

Ordem | Tipo | Tamanho
------|------|--------
  1°  | efi  | 500MB  
  2°  | swap | 2GB    
  3°  | root | 70GB   
  4°  | home | Todo o resto*

Em sistemas **Legacy** faça as seguintes partiçoẽs (Lembrando que a ordem não importa) e selecione "Finalizar o particionamento e escrever as mudanças no disco", depois clique em "Continuar":

Ordem | Tipo | Tamanho
------|------|--------  
  1°  | swap | 2GB    
  2°  | root | 70GB   
  3°  | home | Todo o resto*

![](imagens/escreverdebian11.webp)


Agora aguarde a instalação do sistema básico:

![](imagens/basicodebian11.webp)


Nessa tela selecione "Não" e clique em "Continuar":

![](imagens/midiaadicionaldebian11.webp)


Selecione o país para que o instalador liste os repositórios mais próximos:

![](imagens/repositoriodebian11.webp)


Agora selecione "deb.debian.org" e clique em "Continuar":

![](imagens/1-repositoriodebian11.webp)


Deixe em branco e clique em "Continuar" caso não saiba ou não utilize proxy:

![](imagens/proxydebian11.webp)


Agora marque "Não" e clique em "Continuar":

![](imagens/telemetriadebian11.webp)


Selecione "Utilitários de sistema padrão" e clique em "Continuar":

![](imagens/softwaredebian11.webp)


Selecione "Sim" e clique em "Continuar" para instalar o grub:

![](imagens/grubdebian11.webp)


Agora selecione o disco que será instalado o grub:

![](imagens/discogrubdebian11.webp)


Após a finalização clique em "Continuar" e retire seu pendrive:

![](imagens/fimdebian11.webp)


### Pós instalação


Agora você cairá na tela do terminal, para terminar a instalação do sistema, primeiramente logue-se como usuário **root** após isso, digite os seguintes comandos:


**O 1° para instalar o git:**

  apt install git -y


**O 2° para clonar esse repositório:**

  git clone https://github.com/PEDROTUBER44/mild.git


**O 3° para entrar dentro da pasta do repositório clonado:**

  cd mild


**O 4° para dar permissão de execução para o arquivo de instalação do mild:**

  chmod +x install.sh


**O 5° para instalar o mild em seu PC:**

  ./install.sh


****


Agora escolha qual ambiente gráfico que será instalado em seu PC:


### Lxde

  mild --install-debian-lxde


### Lxqt

  mild --install-debian-lxqt


### Xfce

  mild --install-debian-xfce


### Gnome

  mild --install-debian-gnome


### Mate

  mild --install-debian-mate


### Bspwm

  mild --install-debian-bspwm


### Kde Plasma

  mild --install-debian-kdeplasma


### Cinnamon

  mild --install-debian-cinnamon


Após isso você terá um ambiente desktop clean sem aplicativos redundantes no seu **Debian 11 !**


****

## ArchLinux

Primeiramente baixe do site oficial a iso do ArchLinux, após isso faça um pendrive bootavel, com a iso do archlinux e configure na BIOS, para dar boot pelo pendrive. Após isso você verá essa tela e nela você deverá selecionar "Arch Linux install medium" para instalar o ArchLinux:

![](imagens/inicioarchlinux.webp)


Agora instale o Arch Linux como você costuma instalar só que na hora que você for instalar a interface gráfica clone e instale como **root** o mild, com os seguintes comandos:


**O 1° para instalar o git:**

  pacman -Sy git glibc -y


**O 2° para clonar esse repositório:**

  git clone https://github.com/PEDROTUBER44/mild.git


**O 3° para entrar dentro da pasta do repositório clonado:**

  cd mild


**O 4° para dar permissão de execução para o arquivo de instalação do mild:**

  sudo chmod +x install.sh


**O 5° para instalar o mild em seu PC:**

  ./install.sh


****

Agora escolha qual ambiente gráfico que será instalado em seu PC:


### Lxde

  mild --install-arch-lxde

![](imagens/archlinux-lxde.webp)

---

### Lxqt

  mild --install-arch-lxqt

![](imagens/archlinux-lxqt.webp)

---

### Xfce

  mild --install-arch-xfce

![](imagens/archlinux-xfce.webp)

---

### Gnome

  mild --install-arch-gnome

![](imagens/archlinux-gnome.webp)

---

### Mate

  mild --install-arch-mate

![](imagens/archlinux-mate.webp)

---

### Kde Plasma

  mild --install-arch-kdeplasma

![](imagens/archlinux-kdeplasma.webp)

---

### Cinnamon

	mild --install-arch-cinnamon

![](imagens/archlinux-cinnamon.webp)

Após isso você terá um ambiente desktop clean sem aplicativos redundantes no seu **Arch Linux !**

****

MILD - Minimal Install Linux Desktop
