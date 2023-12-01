import re

str2num = {
    "one": "o1e",
    "two": "t2o",
    "three": "t3e",
    "four": "f4r",
    "five": "f5e",
    "six": "s6x",
    "seven": "s7n",
    "eight": "e8t",
    "nine": "n9e",
}

def replace_words(text):
    for k, v in str2num.items():
        text = text.replace(k, v)
    return text

def calibration(text):
    return sum(int(l[0] + l[-1]) for l in re.sub(r"[A-z]", "", text).split("\n"))

text = open("day1_1_input.txt").read()
print(calibration(replace_words(text)))