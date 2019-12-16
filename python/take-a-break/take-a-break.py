#!/usr/bin/env python3
import sys
import tkinter as tk
from tkinter import font as tkFont


def sec_to_time(sec):
    mins, secs = divmod(sec, 60)
    return f'{mins:02d}:{secs:02d}'


class TakeABreak(tk.Tk):
    def __init__(self):
        super().__init__()

        self.title('Take a break')

        s_height = self.winfo_screenheight()
        s_width = self.winfo_screenwidth()
        height, width = 200, 300
        x = s_width // 2 - width // 2
        y = s_height // 2 - height // 2 - s_height // 20
        self.geometry(f'{width}x{height}+{x}+{y}')

        font16 = tkFont.Font(size=16)

        label = tk.Label(self, text='Take a break!')
        label.config(font=font16)
        label.pack(pady=10)
        self.head = label
        self.remain = 0
        self.show_time = tk.Label(self, text=sec_to_time(self.remain))
        self.show_time.config(font=font16)
        self.show_time.pack()

        self.close = tk.Button(
            self, text='Close',
            command=self.destroy,
            font=tkFont.Font(size=15))
        self.close.pack(pady=10)

    def countdown(self, sec=None):
        if sec is not None:
            self.remain = sec
        if self.remain <= 0:
            self.head.config(text='Time is up!')
            self.show_time.config(text='00:00')
            self.after(3000, self.destroy)
            return
        self.remain -= 1
        self.show_time.config(text=sec_to_time(self.remain))
        self.after(1000, self.countdown)


if __name__ == '__main__':
    try:
        minutes = int(sys.argv[1])
    except Exception:
        minutes = 3
    app = TakeABreak()
    app.attributes('-type', 'dialog')
    app.countdown(60 * minutes)
    app.mainloop()
