# Modulus Hashing

Modulus hashing is a technique for assigning data to servers or buckets. It works like this:

1. Take the key or value you want to assign.
1. Run it through a hashing function to get a hash value. This scrambles the key into a random-looking integer.
1. Take the modulus of the hash value by dividing by the total number of servers.
1. The modulus becomes the index of the server to assign the data to.

For example, if we have 4 servers and hash("data") is 57, then 57 % 3 = 0. This means "data" gets assigned to server #0.

![diagram of modulus hashing](https://excalidraw.com/#json=lCwXToLQAyZvylB-KUecg,Tfa9wTbMb2JjQd1MZOi9CA)

# Limitations

1. Every time we add/remove a server the data gets remapped to different servers.
1. Can still lead to uneven data distribution.
