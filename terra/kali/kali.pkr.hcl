packer {
  required_version = ">= 1.9.2"

  required_plugins {
    vmware = {
      source  = "github.com/hashicorp/vmware"
      version = "~> 1"
    }
  }
}

source "vmware-iso" "kali" {
  # -- | Kali Linux 2023.2 (64-bit)
  iso_url              = "https://cdimage.kali.org/kali-2023.2/kali-linux-2023.2a-installer-amd64.iso"
  iso_checksum         = "sha256:4aeaac60c69fb7137beaaef1fa48c194431274bcb8abf2d9f01c1087c8263b6a"

  # -- | VM settings
  output_directory     = "VMs/Kali Dragon"
  vm_name              = "Kali Dragon"
  guest_os_type        = "debian11-64"
  snapshot_name        = "Fresh Install!"
  cpus                 = 2
  cores                = 2
  memory               = 8192
  disk_size            = 53687
  sound                = true
  usb                  = true

  # -- | VMTools
  tools_upload_flavor  = "linux"

  # -- | Installation
  http_directory       = "./cfg"
  boot_wait            = "10s"
  boot_command         = [
    "<esc><wait>",
    "install preseed/url=http://{{ .HTTPIP }}:{{ .HTTPPort }}/preseed.cfg",
    " auto debian-installer=en_US locale=en_US kbd-chooser/method=us console-keymaps-at/keymap=us keyboard-configuration/xkb-keymap=us",
    " netcfg/get_hostname=kali netcfg/get_domain=local fb=false",
    " debconf/frontend=noninteractive console-setup/ask_detect=false <wait>",
    "<enter><wait>"
  ]

  # -- | Post-installation
  shutdown_command     = "echo 'hacker' | sudo -S shutdown -P now"
  communicator         = "ssh"
  ssh_username         = "hacker"
  ssh_password         = "hacker"
  ssh_timeout          = "30m"
  cleanup_remote_cache = true
}

build {
  sources = ["sources.vmware-iso.kali"]
}