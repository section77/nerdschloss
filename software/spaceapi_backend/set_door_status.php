<?php

$PW_HASH = '<sha512-hash>';

if (!isset($_SERVER['PHP_AUTH_USER'])) {
    header('WWW-Authenticate: Basic realm="section77"');
    header('HTTP/1.0 401 Unauthorized');
    echo 'Unauthorized';
    exit;
}

if ($_SERVER['REQUEST_METHOD'] !== 'PUT') {
    echo 'Invalid Method';
    exit;
}

$user = $_SERVER['PHP_AUTH_USER'];
if ($user !== 'nerdschloss') {
    echo 'Invalid user';
    exit;
}

$password = $_SERVER['PHP_AUTH_PW'];
if (hash('sha512', $password) !== $PW_HASH) {
    echo 'Invalid password';
    exit;
}

$newDoorStatus = $_GET['status'] ?? false;
if (!in_array($newDoorStatus, ['open', 'closed'])) {
    echo 'Status should be "open" or "closed"';
    exit;
}

file_put_contents('door_status.txt', $newDoorStatus);
echo 'Door status has been set to ' . $newDoorStatus;
