# tune 

[![Build Status](https://travis-ci.com/wildpumpkin/tune.svg?branch=master)](https://travis-ci.com/wildpumpkin/tune)

An app to search for songs and videos. 🎧 

## Getting Started

Download the app for your environment. Get a [Genuis Access Key](https://genius.com/api-clients) and a [Google API key](https://console.developers.google.com/) with YouTube Data API v3 enabled.

Configure tune

```
./tune configure
```

To search for lyrics

```console
$ ./tune lyrics "give us a kiss" --artist "the fratellis"
Found 9 results
0: Henrietta by The Fratellis
1: Stacie Anne by The Fratellis
2: Shotgun Suge vs. Nu Jerzey Twork by URLtv (Ft. Nu Jerzey Twork & Shotgun Suge)
3: Elemental Choppers 6 by Echo (956) (Ft. Big Switch, Dvagoh, ETR, Feal, GANEEN!, Leechy Boi, Lyr1c, M0N5T3RRR, Mistery, MultiPerson, MurrK, Neebah, Nibiru Dyve, Nuova, ​sAucEpAn, Shad (BR), TheJackal, Tokumei, Tuono & Xvxfallen)
4: Woman in the Nineteenth Century (Chap. 1) by Margaret Fuller
5: The Vampire, His Kith And Kin - Chapter 1 by Montague Summers
6: February 2020 Singles Release Calendar by Genius
7: Mateo Falcone by Prosper Merimee
8: The Godfather 2 Scene 6 by The Godfather (Film)
```


To search for videos

```console
$ ./tune video "Now or Never Now by Metric"
Found 5 results

Title: Metric - Now or Never Now (Official Video)
 Desc: "Now or Never Now" official music video from the new Metric album "Art of Doubt," out now. ilovemetric.com CREDITS Directed by Lorraine Nicholson & Emily ...
Watch: https://www.youtube.com/watch?v=NC8MfulGMXE

Title: Metric - Now or Never Now - Art of Doubt [Official Audio]
 Desc: Now or Never Now from METRIC's new album, ART OF DOUBT, out now. ilovemetric.com.
Watch: https://www.youtube.com/watch?v=U7DUOcCgmpU

Title: Metric - Now or Never Now
 Desc: From the Verge Music Lab at Trench Recordings: SiriusXM Canada presents Emily Haines and Jimmy Shaw from Metric performing "Now or Never Now"
Watch: https://www.youtube.com/watch?v=X1H6TKNB3SE

Title: Now or Never Now (Radio Edit)
 Desc: Provided to YouTube by BMG Rights Management (US) LLC Now or Never Now (Radio Edit) · Metric Now or Never Now ℗ 2018 MMI/Crystal Math Music ...
Watch: https://www.youtube.com/watch?v=YfFpnVjXtvk

Title: Metric performing "Now or Never Now" live on KCRW
 Desc: Celebrating their 20th year as a band, Metric's latest album Art of Doubt brings their guitar laden songs back to the forefront. Watch the full session here: ...
Watch: https://www.youtube.com/watch?v=y7cSbv9fCcc
```


Enjoy!

