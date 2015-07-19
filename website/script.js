if (isHashConfirmed() == "true") {
	runstatus();
} else {
	runlogin();
}

function isHashConfirmed() {
	hash = window.localStorage.getItem("hash");
	if (hash != null) return "true";
	return "false";
}

function runlogin() {
	var pContent = document.getElementById('pContent');
	console.log("hash is NOT ok. Loading login");
	pContent.innerHTML =	
				'<form onsubmit="return dologin()" action="" method="get"><div data-role="fieldcontain">' +
				'<label for="user">Brukernavn</label>' +
				'<input type="text" id="user" name="user" placeholder="brukernavn" required />' +
				'</div>' +
				'<div data-role="fieldcontain">' +
				'<label for="password">Passord</label>' +
				'<input type="password" id="password" name="password" placeholder="passord" required />' +
				'</div>' +
				'<input type="submit" name="login" value="Logg inn"></div></form>';

	pContent.innerHTML +=	'<br><label name="status" id="status"></label>';
}

function dologin() {
	var user = document.getElementById('user').value;
	var pwd = document.getElementById('password').value;
	var hash = CryptoJS.SHA1(user+":"+pwd).toString(CryptoJS.enc.Hex);
	var socket = new WebSocket('ws://localhost:8876', 'rust-websocket');
	var returnval = "";
	var returnedhash = "";
	var count = 0;
	var idletime = 50;
	var maxtime = 1000;
	socket.onerror = function(event) {
		console.log('error');
		returnval = "done";
	};
	socket.onopen = function(event) {
		//object must be formatted so that the json output format is correct for rust json
		var msg = {"hash":hash, "action":"Login"};
		var msgjson = JSON.stringify(msg);
		socket.send(msgjson);
	};
	socket.onmessage = function(event) {
		var msg = JSON.parse(event.data);
		returnedhash = msg.hash;
		returnval = "done";
	};
	var waitforhash = function(callback) {
		setTimeout(function() {
			if (returnval == "done") {
				console.log("done waiting for reply");
				callback();
				return;
			} else {
				count++;
				if (count*idletime>=maxtime) {
					callback();
					return;
				}
				console.log("Waiting for hash reply");
				waitforhash(callback);
			}
		}, idletime);
	};
	waitforhash(function() {
		if (returnedhash == hash) {
			//hash is ok
			//store hash and reload
			window.localStorage.setItem("hash",hash);
			window.location.reload(true);
			console.log("hash is ok");
		} else {
			//hash is not ok
			var statuslabel = document.getElementById('status');
			if (socket.readyState == 1) {
				statuslabel.innerHTML = 'Feil passord';
			} else {
				statuslabel.innerHTML = 'Server nede';
			}
			console.log("hash is not ok");
		}
		socket.close();
	});
	return false;
}

function runstatus() {
	console.log("hash is ok. Loading status");
	var status = "";
	var hash = window.localStorage.getItem("hash");
	var socket = new WebSocket('ws://localhost:8876', 'rust-websocket');
	var returnval = "";
	var count = 0;
	var idletime = 50;
	var maxtime = 1000;
	socket.onerror = function(event) {
		console.log('error');
		returnval = "done";
	};
	socket.onopen = function(event) {
		//object must be formatted so that the json output format is correct for rust json
		var msg = {"hash":hash, "action":"RequestStatus"};
		var msgjson = JSON.stringify(msg);
		socket.send(msgjson);
	};
	socket.onmessage = function(event) {
		status = JSON.parse(event.data);
		returnval = "done";
	};
	var waitforstatus = function(callback) {
		setTimeout(function() {
			if (returnval == "done") {
				console.log("done waiting for reply");
				callback();
				return;
			} else {
				count++;
				if (count*idletime>=maxtime) {
					callback();
					return;
				}
				console.log("Waiting for reply");
				waitforstatus(callback);
			}
		}, idletime);
	};
	waitforstatus(function() {
		if (status == "") {
			//did not get a status. Show error message.
			console.log("Did not get a status");
			pContent.innerHTML = "<label>Problem med server. Fikk ingen status</label>";
		} else {
			//did get a status. populate
			console.log("Status received");
			pContent.innerHTML = "<label>Fikk en status</label>";
			populate(status, pContent);
		}
		socket.close();
	});
	return false;
}

function populate(status, content) {
	console.log("Populating");
}
