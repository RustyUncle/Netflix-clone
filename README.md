# Netflix-clone
<h2>A Netflix clone made using Rust, Actix-web, and Postgres </h2>

![pic1](https://user-images.githubusercontent.com/46470297/225446107-7c35710f-d76a-4415-81c7-188265a9e150.PNG)
![pic2](https://user-images.githubusercontent.com/46470297/225446169-1a4b859c-3ec9-4704-9180-3740fe8346f8.jpg)
![pic3](https://user-images.githubusercontent.com/46470297/225446231-92722dc2-dfbc-4b38-872c-5ce00c33e9ef.jpg)
![pic4](https://user-images.githubusercontent.com/46470297/225446261-3a09693b-2ef1-48b0-a9d0-0cc732340367.jpg)

Install crates where there are inside Cargo.toml</br>

Remove env.env file name make it like .env file and push your Postgres database url</br>
For example: DATABASE_URL=postgres://YOUR USERNAME:YOUR PASSWORD@localhost/rustflix_db</br>

Import rustflix_db.sql to Postgres</br>

There are a few predefined preview movie and thumbnail names inside rustflix_db.sql</br>
so use them as adding movie files to entities/previews/ - for example: "entities/previews/extraction.mp4"</br>

<h4>Predifined movie list:</h4></br> 
<li>extraction.mp4</li>
<li>6_underground.mp4</li>
<li>against_the_ice.mp4</li>
<li>bird_box.mp4</li>
<li>fantasy_island.mp4</li>
<li>joker.mp4</li>
<li>jumanji.mp4</li>
<li>murder_mystery.mp4</li>
<li>paper_lives.mp4</li>
<li>pulp_finction.mp4</li>
<li>red_notice.mp4</li>
<li>rememberme.mp4</li>
<li>seven_years_intibet.mp4</li>
<li>tenet.mp4</li>
<li>the_godfather2.mp4</li>
<li>the_irishman.mp4</li>
<li>the_king.mp4</li>
<li>the_kissing_booth.mp4</li>
<li>the_man_from_toronto.mp4</li>
<li>the_old_guard.mp4</li>
<li>he_shawshank_redeption.mp4</li>
<li>the_strays.mp4</li>
<li>the_takedown.mp4</li>
<li>thegood_thebad.mp4</li>
<li>thunder_force.mp4</li>
<li>troy.mp4</li>
<li>unlocked.mp4</li></br>

OR add your own list
