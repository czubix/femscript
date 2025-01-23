# Syntax

## Examples

Some code examples for better understanding of the syntax

```femscript linenums="1"
response = await(request("GET", "https://cenzura-api.poligon.lgbt/"));
data = response.json;

embed = Embed{color=randint(0, hex("ffffff"))};
embed.set_thumbnail{borrow(data); url=format("https://cdn.discordapp.com/avatars/{}/{}.png", &data.id, &data.avatar)};
embed.add_field{borrow(data); name="id:"; value=&data.id};
embed.add_field{borrow(data); name="username:"; value=&data.username};

return embed;
```

```femscript linenums="1"
emojis = ["❤️", "🧡", "💛", "💚", "💙", "💜"];
emoji = get(emojis, randint(0, len(emojis)));

comments = [
    "Does this melody echo the quiet moments?",
    "In the silence between notes, what do you hear?",
    "Can a song capture the essence of longing?",
    "Does this track reflect your hidden thoughts?",
    "How does this music intertwine with your solitude?",
    "Is there solace in the rhythms of your day?",
    "Do these lyrics resonate with your unspoken fears?"
];
comment = get(comments, randint(0, len(comments)));

track = lastfm.tracks.0;

if { !track.tags } {
    tags = track.artist.tags;
} else {
    tags = track.tags;
}

embed = Embed {
    borrow(track, tags, emoji, comment);

    fn map_tags(tag) { format("[{}]({})", tag.name, tag.url) }

    title = format("Now Playing: {}", &track.title);
    description = format(
        "{} **Title:** [{}]({})\n**Artist:** [{}]({})\n**Album:** {}\n**Genre:** {}\n**Listeners:** {}\n**Playcount:** {}\n\n**{}**",
        &emoji, &track.title, &track.url,
        &track.artist.name, &track.artist.url,
        &track.album.name,
        join(map(&tags, "map_tags"), ", "),
        &track.listeners,
        &track.playcount,
        &comment
    );
    color = hex("2C2F33");
};

embed.set_author {
    borrow(user, lastfm);

    name = format("{}'s lastfm", &lastfm.user.username);
    icon_url = &user.avatar_url;
    url = format("https://last.fm/user/{}", &lastfm.user.username);
};

embed.set_thumbnail {
        borrow(track);

    url = &track.image.3.url;
};

embed.add_field {
    borrow(track);

    name = "Track Plays";
    value = format("🔄 {} times", &track.scrobbles);
    inline = true;
};

embed.add_field {
    borrow(track);

    name = "Artist Plays";
    value = format("🎸 {} times", &track.artist.stats.userplaycount);
    inline = true;
};


embed.add_field {
    borrow(track);

    name = "Listeners";
    value = format("👥 {} listeners", &track.listeners);
    inline = true;
};

embed.set_footer {
    borrow(lastfm);

    text = format("📊 Total Scrobbles: {} | Powered by Last.fm", &lastfm.user.scrobbles);
    icon_url = "https://www.last.fm/static/images/lastfm_logo.png";
};

return embed;
```

```femscript linenums="1"
command {
    name = "twitter";
    aliases = ["tw"];
    arguments = {
        name = "str";
    };
};

coro = request{borrow(name); method = "GET"; url = format("https://api.fxtwitter.com/{}", &name); headers = {"user-agent" = "testybot/1.0";};};
data = await(coro);
json = data.json;

if { data.status == 404 } {
    return "User not found";
} else {
    embed = Embed{color=hex("1a8cd8")};
    embed.set_title(format("@{}", json.user.screen_name));
    embed.set_thumbnail{borrow(json); url = &json.user.avatar_url};
    embed.set_description(json.user.description);
    embed.add_field{borrow(json); name = "Stats"; value = format(">>> **Followers:** {}\n**Following:** {}\n**Likes:** {}\n**Tweets:** {}", str(&json.user.followers), str(&json.user.following), str(&json.user.likes), str(&json.user.tweets));};

    return embed;
}
```

---

## Returning Values

In Femscript, you can return an object without using the `return` keyword. Instead, the last line of the function (or block of code) will be implicitly returned.
You can implicitly return an object without the return keyword

```femscript linenums="1"
embed = Embed();
embed
```

```femscript linenums="1"
"Hello world!"
```

---

## Arguments and Keyword Arguments

Femscript distinguishes between two main types of passing data to functions: positional arguments (args) and keyword arguments (kwargs). Both are easy to use and allow flexible data passing.

### Positional Arguments

Positional arguments are passed traditionally, where the order of the arguments matters. Example:
```femscript linenums="1"
request(method, url);
```
Here, `method` and `url` are arguments passed to the `request` function. The order of arguments is important, and they must be passed in the same order as declared in the function definition.

### Keyword Arguments

Keyword arguments allow you to pass arguments in the form of key-value pairs. This way, you don't have to worry about the order of the arguments. It's especially useful when a function takes many arguments, or when you want to modify only specific ones.
```femscript linenums="1"
request{method=method; url=url};
```

In this case, the arguments are passed as key-value pairs. `method` and `url` are the keys, and their values are passed to the `request` function.