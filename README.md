<p align="center">
  <img width="100%" src="https://user-images.githubusercontent.com/35466834/224575794-d73000f2-c45d-4346-a37c-c1eaaf295513.png">
</p>

<h1 align="center"> rpower </h1>
<p align="center">Power menu written in Rust, because I've grown bored of rofi and needed something simple, customizable and (kinda) light.</p>

## Prequesities
- [Vulkan](https://wiki.archlinux.org/title/Vulkan) (because of [this issue](https://github.com/iced-rs/iced/issues/1103))
- [Cargo](https://github.com/rust-lang/cargo)

## Instalation
To build as release and initialize cofig directory:
```shell
$ make
```

## Configuration
All configuration is located in $HOME/.config/rpower

### [config.toml](./config/config.toml)
#### Window
- `width` => Window width
- `height` => Window height
- `background` => Color of background behind buttons (format: `[r, g, b]`)

#### Buttons
- `commands`      => List of commands coresponding to each button
- `icon_names`    => List of icon filenames (without extension) in the icons folder
- `icon_colors`   => List of icons colors
- `normal_colors` => List of button background colors when not hovered
- `hover_colors`  => List of button background colors when hovered

### [Button Icons](./config/icons)
- You can change any button to any svg inside the icon folder using `icon_names`
- Included icons come source:
  - [Poweroff](https://www.svgrepo.com/svg/332492/poweroff)
  - [Reboot](https://www.svgrepo.com/svg/487723/reload-ui-2?edit=true)
  - [Suspend](https://www.svgrepo.com/svg/353055/controller-paus)
  - [Lock](https://www.svgrepo.com/svg/505417/lock-on)
