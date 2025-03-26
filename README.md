# [⚒️ Embassy workshop](https://rust.ipworkshop.ro/docs/embassy)

![PMRust Lab logo](https://gitlab.cs.pub.ro/pmrust/pmrust.pages.upb.ro/-/raw/main/website/static/img/logo.svg?ref_type=heads)




# 🙋‍♂️ What I did at this Rust Workshop

I enhanced my understanding of **Rust** and programming an **MCU** in **embedded systems**
by playing with a Rasberry Pi Pico board, the same used in the [PM Rust laboratories](https://pmrust.pages.upb.ro/).


## [💡 Task 0: Make an LED blink](./src/bin/demo.rs)

Using a **GPIO** pin on the Rasberry Pi and a **jumper wire**
to connect it to the pin of an LED,
I managed to make the LED blink,
either **RED**, **GREEN**, **BLUE** or **YELLOW** (depending on the LED's pin).

Each color corresponds to a totally different LED:
- the pin of the LEDs are labeled on the board, above the screen and in the left side of the Rasberry Pico 2 W
- The LEDs that will blink are found in the left-upper corner of the board, obve the black pin holes and below the switches (sw4-7)

### ⚠️ Attention, Amateurs!


```rs
let mut led = Output::new(p.PIN_4, embassy_rp::gpio::Level::High);
```

Please note that `PIN_4` does NOT correspond to physical pin 4 on the Raspberry Pi Pico.
Instead, it refers to **General Purpose** pin 4 (**GP4**), which is **physically located on pin 6** of the board.

Take a look at the Rasberry Pi Pico's pin layout [here](https://rust.ipworkshop.ro/assets/images/pico2w-pinout-49532ea10ab0caedc6a6f21d1bf504bf.svg).


## [🖰 Task 1: Detect when buttons are pressed](./src/bin/hi.rs)

You've seen the buttons on the top of the board, have you?!
They are labeled **SW4**, **SW5**, **SW6**, **SW7**.

In this task, I implemented an **asynchronous algorithm**
to detect when any of these buttons is pressed
(you can pick any one and press it—it doesn’t matter which!).

- I used the `defmt`, a **Rust framework (crate)** to log the activity on the board.
    It prints messages on the screen of my laptop.
- I used **jumper wires** to connect the Rasberry Pico to the pins of the buttons

    | Rasberry Pi Pico pin     | Board pin |
    | :---:                    | :---:     | 
    | **GP2** (physical pin 4) | SW4       |
    | **GP3** (physical pin 5) | SW5       |
    | **GP4** (physical pin 6) | SW6       |
    | **GP5** (physical pin 7) | SW7       |


    > The buttons's pins (switches **SW4...7**) are located to the left of the colored LEDs pins

- I set the **GPIO** pins as **Input pins** and activated the **pull-up** restistors
  - > The pins will check the buttons' states. 
- Using the **asynchronous** `select` function from the `embassy-futures` crate,
    I waited for any of the four buttons to be pressed,
    and once triggered, I logged the name of my team, *I/O CTL*, using `defmt`


## [🔊🎼 Task 3: Sing my own tune](./src/bin/sing.rs)

![img](https://t4.ftcdn.net/jpg/05/29/82/97/360_F_529829756_PQ1ar8Wel6Xdd29S9XKQkw2Rv4PiZUHD.jpg)

However, computers and microcontrollers only understand
[**digital signals**](https://rust.ipworkshop.ro/assets/images/digital_signal-fb5afef385984f5a23322d339559c85d.png),
which are **square** and **discrete waves**, a sequence of binary values, 0s and 1s, HIGH VOLTAGE/LOW VOLTAGE.

To bridge this gap, the microcontroller (MCU) uses a clever trick:
it rapidly switches between high and low voltages to mimic the frequency of the sound.
This process called [**PWM** (Pulse Width Modulation)](https://rust.ipworkshop.ro/assets/images/pulse-width-modulation-signal-diagrams-average-e8f71f3620c486efcdefc7756a750c3b.png)
**simulates an analog signal**,
allowing the MCU to produce music using digital signals!

Using **PWM**, I managed to play the entire **solfège scale** (**Do-Re-Mi-Fa-Sol-La-Si-Do**):



- Hardware:
  - I connected the General Purpose pin 2 (**GP2**) of the Rasberry Pi Pico to the buzzer's pin with a jumper wire
- Software:
  - Configure and **activate buzzer** and the **PWM** pin associated to it
  - For each note in the octave:
    - Compute its duration (so I know how much I should adjust the PWM)
    - Compute `top`, how long one full PWM cycle lasts:
        ```rs
        top = CLOCK_FREQ / (PWM_DIV * note_frequency) - 1
        ```
    - Reconfigure and PWM and activate the buzzer using this `top` variable
    - Play the sound for 90% of the time
    - Turn the buzzer off for the rest of 10% of the time (before playing the next note)



# Useful links

- [Rasberry Pi Pico pins layout](https://rust.ipworkshop.ro/assets/images/pico2w-pinout-49532ea10ab0caedc6a6f21d1bf504bf.svg)
- [Board](https://rust.ipworkshop.ro/assets/images/lab_board-3fa09dc706781d6e6c7126212ca9d240.png)
- [Rust Workshop - embassy](https://rust.ipworkshop.ro/docs/embassy)
- [PM Rust](https://pmrust.pages.upb.ro/)


# How to use this repo

1. **Hardware Setup**:
  use **jumper wires** to connect Rasberry Pi's pins
  to the corresponding pins of your desired component
2. Connect the board to your personal computer via **USB**
3. **Flash the program**:
  to execute a specific task,
  run the corresponding command from the root folder of this repository


| Task | CLI command to flash |
| :--- | :--- |
| **Task 0**: Make an LED blink | `cargo run --bin demo` |
| **Task 1**: Detect when buttons are pressed | `cargo run --bin hi` |
| **Task 3**: Sing my own tune | `cargo run --bin sing` |

