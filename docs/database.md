Password Manager Notes
======================

Parts of the database (v1)
--------------------------

### User

Each database can have multiple users, and each user can have as many entries as they want.

Parts of a User:

- Store hash of Master Password (probably unnecessary, requires some testing)
- Parameters, such as memlimit and opslimit for generating the master key, along with the user's salt
- UUID to identify the user
- Encrypted private key for sharing (encrypted with the master password)
- Public key for sharing (doesn't require encryption)

### Entries 

Each entry is a record of a site. An entry is identified by a UUIDv4. An entry is made up of two part, `meta.json` and `data.json`.

List of things in a Entry's metadata:

- UUIDv4 for identification (string)
- Name (string)
- Website (string)
- Folder (string)
- Favorite (bool)

List of things in an Entry's data:

- UUIDv4 for identification (string)
- Username (string)
- Password (string)
- Comments (string)
- Extra fields (array of tuples: label(string), content(string))
		- These could be useful to track specific things about each site in a more granular manner than just storing them in the comments box
- Attachments (array of UUIDs: UUIDv4(string))
		- Files can be attached to an entry, and they will be encrypted and stored separately (and renamed, so as not to leak any metadata)

### Attachments

Attachments are stored separate from entries in their own directory. Each attachment has it's own metadata.

List of things in an Attachment's metadata:

- UUIDv4 for identification (string)
- Original name
- Filetype

### History

Each entry will have a log of its history saved along side of it. This will allow the user to look back at previous passwords and other fields. A history is just simpily  a collection of history records.

List of things in a Record:

- Type (enum, either an addition, deletion, or modification)
- Field name that was touched
- Time of modification (in UTC, for easier/standard comparison)
- Value prior to change
- Value after change

Sharing
-------

Sharing must be done in a distributed fashion, as the database won't use a central server for communication. Information about an entry is shared through a info file.

Parts of an Info File:

- User UUIDv4 of sender
- Entry UUIDv4
- List of exported fields and their values

The Info file can either be a) not encrypted (like 1Password), b) encrypted with a Pre-shared key (i.e. an answer to a simple question both parties know), or c) encrypted with user public and private keys (this requires sharing ahead of time)

Encryption
----------

The user creates a master password to encrypt and decrypt their data. A password by itself is a weak key, so to make it stronger, the master password will be run through a key derivation function (scrypt), to create the master key.
This master key can then be used to decrypt each entry's metadata, content, it's history, it's attachments, and the user's private key.

File System Structure
---------------------

- root/
	- dbinfo.json (stores things like db version number, anything else that will be necessary)
	- users/
		- <friendly username>/
			- userinfo.json (to store user UUID, opslimit, memlimit, salt, and maybe hash for key derivation)
			- private.key (stored encrypted on disk)
			- public.key (not stored encrpyted on disk)
			- attachments/
				- <attachment UUID>/
					- meta.json
					- file (named with the UUID of the attachment)
				- ...
			- db/
				- <entry UUID>/
					- meta.json (keeps metadata separate from regular entries)
					- data.json (stores all the content about the entry)
					- history.json (stores all the history about the entry)
					- ...
				- ...
			- friends/
				- <user UUID>/
					- friendinfo.json (maybe store information about friend, like name, etc.)
					- public.key (public key for a user)
				- ...
		- ...
