from mnemonic import Mnemonic

good_mnemonic = "monkey frog radio rubber weekend doctor leave project stay junior dad catch"
bad_mnemonic = "frog radio rubber weekend doctor leave project stay junior dad catch monkey"
print("good mnemonic:", Mnemonic().check(good_mnemonic))
print("bad mnemonic:", Mnemonic().check(bad_mnemonic))
