import hashlib

def verify_model_safety(model_path: str, expected_hash: str) -> bool:
    BUF_SIZE = 65536
    sha256 = hashlib.sha256()
    
    with open(model_path, 'rb') as f:
        while True:
            data = f.read(BUF_SIZE)
            if not data:
                break
            sha256.update(data)
    
    return sha256.hexdigest() == expected_hash
