#!/usr/bin/env python

from mattermostdriver import Driver


def main(*args):
    url = args[0]
    login_id = args[1]
    api_token = args[2]
    scheme = args[3]
    port = args[4]
    state = args[5]

    driver = Driver(
        {
            "url": url,
            "login_id": login_id,
            "token": api_token,
            "scheme": scheme,
            "port": int(port),
        }
    )
    driver.login()

    team = driver.teams.get_team_by_name("section77")
    channel = driver.channels.get_channel_by_name(team["id"], "clubstatus")

    message = ""
    name = ""
    if state == "true":
        message = "Die Section77 ist offen"
        name = "Status: Offen"
    elif state == "false":
        message = "Die Section77 ist geschlossen"
        name = "Status: Geschlossen"

    # send message
    _post = driver.posts.create_post({"channel_id": channel["id"], "message": message})
    # update room name
    driver.channels.patch_channel(
        channel["id"], {"id": channel["id"], "display_name": name}
    )

    driver.logout()
