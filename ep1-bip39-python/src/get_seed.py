import hashlib

from mnemonic import Mnemonic

mnemonic = Mnemonic(language="english")
mnemonic_sentence = "monkey frog radio rubber weekend doctor leave project stay junior dad catch"

passphrase = "my-secret"
entropy = mnemonic.to_entropy(mnemonic_sentence)
seed = mnemonic.to_seed(mnemonic_sentence, passphrase=passphrase)

print("mnemonic sentence:", mnemonic_sentence)
print("passphrase:", passphrase)
print("seed:", seed.hex())

# Generate the seed from the mnemonic sentence and passphrase using the PBKDF2 function
passphrase = "mnemonic" + passphrase
seed = hashlib.pbkdf2_hmac("sha512", mnemonic_sentence.encode("utf-8"), passphrase.encode("utf-8"), 2048)
print("seed:", seed.hex())
