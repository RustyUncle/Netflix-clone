$(document).scroll(function() {
    var isScrolled = $(this).scrollTop() > $(".topBar").height();
    $(".topBar").toggleClass("scrolled", isScrolled);
})

function volumeToggle(button) {
    var muted = $(".previewVideo").prop("muted");
    $(".previewVideo").prop("muted", !muted);

    $(button).find("i").toggleClass("fa-volume-mute");
    $(button).find("i").toggleClass("fa-volume-up");
}

function previewEnded() {
    $(".previewVideo").toggle();
    $(".previewImage").toggle();
}

function goBack() {
    window.history.back();
}

function startHideTimer() {
    var timeout = null;
    
    $(document).on("mousemove", function() {
        clearTimeout(timeout);
        $(".watchNav").fadeIn();

        timeout = setTimeout(function() {
            $(".watchNav").fadeOut();
        }, 2000);
    })
}



function initVideo(videoid, username) {
    startHideTimer();
    setStartTime(videoid, username);
    updateProgressTimer(videoid, username);
}

function updateProgressTimer(videoid, username) {
    addDuration(videoid, username);

    var timer;

    $("video").on("playing", function(event) {
        window.clearInterval(timer);
        timer = window.setInterval(function() {
            updateProgress(videoid, username, event.target.currentTime);
        }, 3000);
    })
    .on("ended", function() {
        setFinished(videoid, username);
        window.clearInterval(timer);
    })
}

function addDuration(videoid, username) {
    $.post("/ajax/adduration", { videoid: videoid, username: username }, function(data) {
        if(data !== null && data !== "") {
            alert(data);
        }
    })
}

function updateProgress(videoid, username, progress) {
    $.post("/ajax/updateduration", { videoid: videoid, username: username, progress: progress }, function(data) {
        if(data !== null && data !== "") {
            alert(data);
        }
    })
}

function setFinished(videoid, username) {
    $.post("/ajax/setfinished", { videoid: videoid, username: username }, function(data) {
        if(data !== null && data !== "") {
            alert(data);
        }
    })
}



function setStartTime(videoid, username) {
    $.post("/ajax/progress", { videoid: videoid, username: username }, function(data) {
        if(isNaN(data)) {
            alert(data);
            return;
        }

        $("video").on("canplay", function() {
            this.currentTime = data;
            $("video").off("canplay");
        })
    })
}

function restartVideo() {
    $("video")[0].currentTime = 0;
    $("video")[0].play();
    $(".upNext").fadeOut();
}

function entityVideo(entityid) {
    window.location.href = "/entity/" + entityid;
}
function watchVideo(videoid) {
    window.location.href = "/watch/" + videoid;
}

function showUpNext() {
    $(".upNext").fadeIn();
}
