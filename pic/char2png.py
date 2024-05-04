import matplotlib.pyplot as plt

if __name__ == '__main__':
    char_ls = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'mon', 'day', 'hou', 'min', 'sec']

    for ch in char_ls:
        if ch.isdigit():
            fig, ax = plt.subplots(figsize=(25.0 / 64, 33.0 / 64), dpi=64)
        else:
            fig, ax = plt.subplots(figsize=(50.0 / 64, 33.0 / 64), dpi=64)

        ax.text(0.5, 0.5, ch, fontsize=25, ha='center', va='center', fontweight='bold')  
        
        plt.axis('off')  
        
        plt.savefig("./{}.png".format(ch))