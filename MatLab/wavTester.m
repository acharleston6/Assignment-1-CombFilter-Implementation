[y,fs] = audioread("sweep.wav");
time = (0:length(y)-1) / fs;


x = y;
g = 0.5;
Delayline=zeros(10,1); % memory allocation for length 10

disp(length(y));

for n=1:length(x)
	y = x + g*Delayline(10);
	Delayline=y;
    Delayline(1:10-1);
    disp(n);
end

figure();
plot(time, y);
xlabel('Time (seconds)');
ylabel('Amplitude');
title('Audio Waveform');

filename = "wavTester.mat";
save(filename)

