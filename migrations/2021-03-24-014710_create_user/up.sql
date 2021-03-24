-- Passwords is `password`

INSERT INTO tenants
VALUES (
    0,
    'superadmin@test.com',
    'superadmin',
    'superadmin',
    '$argon2i$m=4096,t=3,p=1$r/AB1gKgFXdW2VB7PB6IJQ$f82eK85vMatKObM91wObV5kKDCSOP/AJSrcR4SLEmNQ',
    'admin'
);
INSERT INTO tenants
VALUES (
    1,
    'test@test.com',
    'test',
    'test',
    '$argon2i$m=4096,t=3,p=1$r/AB1gKgFXdW2VB7PB6IJQ$f82eK85vMatKObM91wObV5kKDCSOP/AJSrcR4SLEmNQ',
    'tenant'
);