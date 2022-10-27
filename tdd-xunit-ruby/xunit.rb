class TestCase; end

class WasRun < TestCase
  def initialize(name)
    super()
    @name = name
    @was_run = false
  end

  def run
    send(@name)
  end

  def was_run?
    @was_run
  end

  def test_method
    @was_run = true
  end
end
