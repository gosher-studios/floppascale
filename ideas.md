### upload (with dedupe) 
1. client sends file name + file size to manager
2. manager sends OTK to chosen storage node, and approx file size (approx because encryption <3)
3. manager sends OTK and "random" file name (asdasdl.a.png) to client
4. client gets hash of original file, then encrypts file with random file name and posts encrypted file with obfuscated file name
5. after statuscode 200 from storage node to client, client sends manager hash 
6. storage node sends "filename" to manager
7. manager adds "random file name" | filedata to db(/dev/null), UNLESS hash is in the hashes table, if in the hashes table the filedata from the original hash is copied to "random file name" | filedata (this is where original-name) is important, because you need the storagenode filename & the "decryption" filename
8. if duped, send a delete request to storagenode of "file"
9. etc etc etc

#### comments
- OTK also serves as transaction ID
- OTK only valid for one file upload, potentially many files

### download 
1. client sends file name request
2. 




database structure:

snode_id | url, allows_dedup, allows_rebalance, size_left
file-hash | snode_id, snode_file_name
file-name | snode_id, snode_file_name, original_file_name

//todo OTK/transaction temp thing
