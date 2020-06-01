Calculating Utility For Voltage Divider written in Rust (as good as I could):

	Notes To Understand Voltage Divider:

Voltage and Current divider arrangements are the common in electronic circuits. Before selecting the value of resistors, it is necessary to calculate the voltage and current from the output of a resistor chain. So that the circuit will function properly. Here is a simple but useful tip to calculate the voltage and current from resistor chains.

	Voltage divider

By selecting appropriate values of resistors in the divider, it is easy to obtain a fraction of the input voltage. See the voltage divider diagram. The value of the output voltage V Out from the divider R1-R2 will be

V Out = V In x R2 / R1+R2

Suppose we need 5 volts output from a 15 volt power supply.Then we should use the value of R1 twice that of R2. Value of 2K for R1 and 1K for R2 will do the trick

V Out = V in x R2/R1+R2 = 15 X 1000 /2000+1000 = 5 Volt

If the power supply is 12 volts, you will get 4 Volts from the same divider.
If we use 200 ohms for R1 and 100 ohms for R2, same output voltage will be obtained, but current will be more.

	Current divider

By selecting appropriate values of resistors in the divider, it is easy to obtain required output current from the divider.

I Out = I In x R1/R1+R2

Suppose you need a current of 5 mA from a 15 mA input current, you should select the value of R2 twice that of R1. If you select 1 Ohm for R1 and 2 Ohms for R2, then the output current will be 5 mA

I Out = 15mA xR1/R1+R2 = 5 mA

If you use 10 Ohms for R1 and 20 Ohms for R2, same output current of 5 mA will be produced but increased voltage drop will be the result.		Source: https://electroschematics.com/voltage-and-current-divider/

[!screenshot](Voltage-and-Current-Divider.png)
	ToDo:
	-Add the Current Option. 					```Check```
	-Add K M G m option to calculation.
	-Add loop for different calculation.
	-Add Reversible calculation.
	-Maybe also add what is the best Option depending on which resistors are available and test to find the more suitable.
	-And more to come as long as I'll need to obviously...
