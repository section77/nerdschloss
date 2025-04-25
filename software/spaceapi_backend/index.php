<?php

$doorStatus = file_get_contents('door_status.txt');

header('Content-Type: application/json; charset=utf-8');
header('Access-Control-Allow-Origin: *');

echo '{"api":"0.13","api_compatibility":["14"],"space":"section77","logo":"https://section77.de/static/section77_logo_vector.svg","url":"https://section77.de/","location":{"address":"Hauptstraße 1, 77652 Offenburg, Germany","lat":48.4771,"lon":7.9461},"contact":{"twitter":"@section77de","email":"info@section77.de","mastodon":"@section77@chaos.social"},"feeds":{"calendar":{"type":"ical","url":"https://section77.de/section77.ics"}},"issue_report_channels":["email"],"state":{"open":' . ($doorStatus === 'open' ? 'true' : 'false') . '},"ext_ccc":"chaostreff"}';
