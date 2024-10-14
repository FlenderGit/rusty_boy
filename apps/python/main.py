from rusty_boy import RustyBoy

def main():
    gb = RustyBoy("../../roms/zelda.gb", False)
    
    while True:
        gb.run_frame()
        print_screen(gb.get_screen())

def print_screen(screen):
    width = 160
    height = 144
    for i in range(height):
        for j in range(width):
            print(get_char(screen[(i * width + j)*3]), end="")
        print("")

def get_char(value):
    if value == 0:
        return "  "
    elif value == 96:
        return ".."
    elif value == 192:
        return "=="
    else:
        return "##"

if __name__ == "__main__":
    main()