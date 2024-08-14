from mnemonic import Mnemonic

mnemonic = Mnemonic(language="english")
words = mnemonic.generate(strength=128)
print(words)
