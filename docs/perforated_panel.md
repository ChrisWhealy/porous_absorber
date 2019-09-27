# Perforated Panel

A perforated panel of thickness ***t<sub>p</sub>*** with circular holes of radius ***a*** at a spacing ***D*** is mounted above a cavity whose total depth is ***d*** and containing a porous absorber layer of thickness ***t<sub>a</sub>***.

Three absorption curves are calculated:  

* The perforated panel is mounted directly to the porous absorber layer, which in turn is mounted directly to the rigid backing.  I.E. The air gap is zero.

* Perforated Panel -> Porous Absorber -> Air Gap -> Backing 
    ![Perforated Panel 1](../img/perforated_panel1.png) 

* Perforated Panel -> Air Gap -> Porous Absorber -> Backing
    ![Perforated Panel 2](../img/perforated_panel2.png) 

## Default Graph

![Perforated Panel Screen](../img/perforated_panel_screen.png)

## Value Ranges

| Property | Min | Default value | Max |
|---|---|---|---|
| Panel thickness | 1.0 mm | 10 mm | 50.0 mm
| Hole centred every | 2.0 mm | 25.4 mm | 300 mm
| Hole radius | 1.0 mm | 5.0 mm | Half hole centre distance
| Absorber flow resistivity | 100 rayls/m | 16,500 rayls/m | 100,000 rayls/m 
| Absorber thickness | 5 mm | 30 mm | 500 mm
| Air gap | 0 mm | 100 mm | 500 mm
| Graph start frequency | 20 Hz | 62.5 Hz | 100 Hz
| Octave subdivisions | 1 | 1 | 1, 2, 3 or 6


