import secrets

from mnemonic import Mnemonic

entropy = secrets.token_bytes(nbytes=16)  # 16 bytes = 12 words, 32 bytes = 24 words
print("entropy:", entropy.hex())
mnemonic = Mnemonic(language="english")
mnemonic_sentence = mnemonic.to_mnemonic(entropy)
print("mnemonic sentence:", mnemonic_sentence)
