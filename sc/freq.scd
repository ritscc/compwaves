// 周波数取得
(
b = Buffer.alloc(s,512);
~size=16*8;
~cutfreqs = Array.fill(~size,{|i| (s.sampleRate/~size*i).asInteger});

c = Bus.control(s,~size);

x = {
    var raw = FFTSubbandPower.kr(FFT(b,In.ar), ~cutfreqs, square:1, scalemode:1);
    Out.kr(c.index, RunningSum.kr(raw/(~cutfreqs[1]), 10)/10);
}.play(s,addAction: \addToTail);
)

// OSC送信
(
a = NetAddr.new("127.0.0.1", 2020);
v = Int8Array.fill(~size/2,0);
o = Task({
    loop{
        c.getn(~size/2,{|val|
            {

				(val.log10 * 10).do({|e,i|
					if(e < -96 || e.isNaN) {v[i] = -96} {v[i] = e.asInteger};

				});

                a.sendMsg("/freq", v);

            }.defer;
        });
        0.032.wait;
    }
}).start;
)

(
o.stop;
x.free;
)
