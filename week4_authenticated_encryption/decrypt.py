import urllib.request
import urllib.parse
import binascii
import sys

TARGET = 'http://crypto-class.appspot.com/po?er='

class PaddingOracle(object):
		def query(self, q):
			target = TARGET + q
			print("target =", target)
			req = urllib.request.Request(target)
			try:
				f = urllib.request.urlopen(req)          # Wait for response
			except urllib.error.HTTPError as e:          
				print("We got: %d" % e.code)       # Print response code
				assert(e.code == 404 or e.code == 403)
				if e.code == 404:
					return True # good padding
			return False # bad padding

if __name__ == "__main__":
	po = PaddingOracle()

	cyphertext = "f20bdba6ff29eed7b046d1df9fb7000058b1ffb4210a580f748b4ac714c001bd4a61044426fb515dad3f21f18aa577c0bdf302936266926ff37dbf7035d5eeb4"
	cyphertext = list(binascii.unhexlify(cyphertext))

	nblocks = int(len(cyphertext) / 16)
	message = [0 for _ in range((nblocks-1)*16)]

	# All blocks excluding the last
	for b in range(nblocks-2):
		block_1 = cyphertext[(b+0)*16:(b+1)*16]
		block_2 = cyphertext[(b+1)*16:(b+2)*16]
		for pad in range(1,17):
			for j in range(pad):
				block_1[15-j] ^= pad
			for g in list(range(65,91)) + list(range(97,123)) + [32]:
				block_1[16-pad] ^= g
				block_s = ''.join("{:02x}".format(x) for x in block_1 + block_2)
				if po.query(block_s):
					break
				block_1[16-pad] ^= g
			for j in range(pad):
				block_1[15-j] ^= pad
			message[(b+1)*16 - pad] = g

	# Last block must be processed different since it has a valid path
	block_1 = cyphertext[(nblocks-2)*16:(nblocks-1)*16]
	block_2 = cyphertext[(nblocks-1)*16:(nblocks-0)*16]
	for index in range(16):
		block_1[index] ^= 127
		block_s = ''.join("{:02x}".format(x) for x in block_1 + block_2)
		if po.query(block_s):
			start_byte = index
		else:
			block_1[index] ^= 127
			break
		block_1[index] ^= 127
	start_pad = 16 - start_byte
	for j in range(start_byte+1, 16):
		block_1[j] ^= (start_pad-1)

	for pad in range(start_pad,17):
		for j in range(pad):
			block_1[15-j] ^= pad
		for g in list(range(65,91)) + list(range(97,123)) + [32]:
			block_1[16-pad] ^= g
			block_s = ''.join("{:02x}".format(x) for x in block_1 + block_2)
			if po.query(block_s):
				break
			block_1[16-pad] ^= g
		for j in range(pad):
			block_1[15-j] ^= pad

		message[(nblocks-1)*16 - pad] = g

	print(''.join([chr(x) for x in message]))
