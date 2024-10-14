(function(){


    window.addEventListener('load', function(){

        let worker = new Worker('./res/worker.js');

        worker.onmessage = function(e){
            console.log('Worker sent message');
            let gameboy = e.data;
            console.log(gameboy);
        }

        document.getElementById('file').addEventListener('change', function(e){
            let file = e.target.files[0];
            console.log("Outside", file);
            worker.postMessage(file);            
        });

    
        let canvas = document.getElementById('canvas');
        
        // Canvas on fullscreen on f11
        document.addEventListener('keydown', function(e){
            if(e.key === 'F11'){
                canvas.requestFullscreen();
            }
        });

        let gamepad = null;
        let previous_keys = Array(8).fill(0);
        let interval = null;

        this.window.addEventListener('gamepadconnected', function(e){
            gamepad = e.gamepad;
            interval = setInterval(fn_interval, 1000/60);
        });

        this.window.addEventListener('gamepaddisconnected', function(e){
            gamepad = null;
            clearInterval(interval);
        });

        // Add listerner for gamepad key press
        const fn_interval = function(){
            if(gamepad){
                let keys = Array(8).fill(0);
                for(let i = 0; i < gamepad.buttons.length; i++){
                    keys[i] = gamepad.buttons[i].pressed;
                }
                if(keys.join('') !== previous_keys.join('')){
                    previous_keys = keys;
                    console.log(keys);
                }
            }
        }
    
    });

}())