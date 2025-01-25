#!/usr/bin/env python

import sys

from mattermostdriver import Driver


def main(*args):
    print(args)

    url = args[0][0]
    login_id = args[0][1]
    api_token = args[0][2]
    scheme = args[0][3]
    port = args[0][4]
    state = args[0][5]

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


if __name__ == "__main__":
    main(sys.argv[1:])
