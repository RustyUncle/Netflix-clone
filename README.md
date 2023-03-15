# Netflix-clone
<h2>A Netflix clone made using Rust, Actix-web, and Postgres </h2>

![pic1](https://user-images.githubusercontent.com/46470297/225446107-7c35710f-d76a-4415-81c7-188265a9e150.PNG)
![pic2](https://user-images.githubusercontent.com/46470297/225446169-1a4b859c-3ec9-4704-9180-3740fe8346f8.jpg)
![pic3](https://user-images.githubusercontent.com/46470297/225446231-92722dc2-dfbc-4b38-872c-5ce00c33e9ef.jpg)
![pic4](https://user-images.githubusercontent.com/46470297/225446261-3a09693b-2ef1-48b0-a9d0-0cc732340367.jpg)

1- remove env.env file name make it like .env file and push your Postgres database url
For example: DATABASE_URL=postgres://YOUR USERNAME:YOUR PASSWORD@localhost/rustflix_db

2- Import rustflix_db.sql to Postgres

3- There are a few predefined preview movie and thumbnail names inside rustflix_db.sql
so use them as adding movie files to "entities/previews/" - For example: "entities/previews/extraction.mp4"

Predifined movie list:
   *extraction.mp4
    6_underground.mp4
    against_the_ice.mp4
    bird_box.mp4
    fantasy_island.mp4
    joker.mp4
    jumanji.mp4
    murder_mystery.mp4
    paper_lives.mp4
    pulp_finction.mp4
    red_notice.mp4
    rememberme.mp4
    seven_years_intibet.mp4
    tenet.mp4
    the_godfather2.mp4
    the_irishman.mp4
    the_king.mp4
    the_kissing_booth.mp4
    the_man_from_toronto.mp4
    the_old_guard.mp4
    he_shawshank_redeption.mp4
    the_strays.mp4
    the_takedown.mp4
    thegood_thebad.mp4
    thunder_force.mp4
    troy.mp4
    unlocked.mp4
  *
OR add your own list
